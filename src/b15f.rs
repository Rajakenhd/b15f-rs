//! This module contains all the structures and functions related to
//! interacting with the B15 on a high level. If you are writing code
//! for the B15, this is the module you want to use.

use std::{process::Command, time::Duration, fmt::Debug, thread::sleep};
use rand::Rng;
use serialport::SerialPort;
use crate::error::Error;

use crate::{request::Request, build_request};

macro_rules! log {
	($text: literal, $($arg:tt)*) => (println!(concat!("[B15F] ", $text), $($arg)*));
	($text: literal) => (println!(concat!("[B15F] ", $text)));
}

macro_rules! log_start {
	($text: literal, $($arg:tt)*) => (print!(concat!("[B15F] ", $text, "... "), $($arg)*));
	($text: literal) => (print!(concat!("[B15F] ", $text, "... ")));
}

macro_rules! log_end {
	($text: literal, $($arg:tt)*) => (println!($text, $($arg)*));
	($text: literal) => (println!($text));
}

/// Structure representing the driver for the board 15
pub struct B15F {
	usart: Box<dyn SerialPort>
}

impl B15F {
	const MSG_OK: u8 = 0xFF;

	/// Creates a new instance of the B15
	/// 
	/// This function will establish a connection to a connected B15 and return
	/// a handle to interact with it. Only one such instance should exist per
	/// program; calling `B15F::new()` more than once might lead to unexpected
	/// behaviour.
	/// 
	/// # Examples
	/// ```
	/// use b15f::B15F;
	/// 
	/// let drv = B15F::new().unwrap();
	/// ```
	pub fn new() -> Result<B15F, Error> {
		let port = B15F::init_connection()?;

		let mut drv =B15F {
			usart: port
		};

		log_start!("Testing connection");
		let mut tries = 3;
		while tries > 0 {
			drv.discard()?;

			match drv.test_connection() {
				Ok(()) => break,
				Err(_) => {} // Do nothing
			};

			tries -= 1;
		}

		if tries == 0 {
			return Err("Connection test failed. Are you using the newest version?".into());
		}
		
		log_end!("Ok!");

		let info = drv.get_board_info()?;
		log!("AVR firmware version: {} built at {} ({})", info[0], info[1], info[2]);

		// let avr_commit_hash = info[3];
		// if avr_commit_hash != COMMIT_HASH {
		// 	log!("Different commit hashes: {} vs {}", avr_commit_hash, COMMIT_HASH);
		// 	return Err("Versions incompatible. Please update the software!".into());
		// }

		Ok(drv)
	}

	fn init_connection() -> Result<Box<dyn SerialPort>, Error> {
		let devices = B15F::get_devices();

		let device = match devices.first() {
			Some(item) => item,
			None => return Err("Failed to find adapter".into())
		};

		log!("Using adapter: {}", device);

		log_start!("Establish connection with adapter");
		
		let port = serialport::new(device, 57_600)
											.timeout(Duration::from_millis(1000))
											.open()?;
		log_end!("Ok!");

		Ok(port)
	}

	/// Yields information about the installed firmware on the B15
	/// 
	/// Returns an array of strings, where each string contains a piece
	/// of information stored on the B15
	/// 
	/// # Examples
	/// ```
	/// use b15f::B15F;
	/// 
	/// let mut drv = B15F::new().unwrap();
	/// 
	/// // Print each bit of information on a new line
	/// drv.get_board_info()
	/// 	.unwrap()
	/// 	.iter()
	/// 	.for_each(|info| println!("{info}"));
	/// ```
	pub fn get_board_info(&mut self) -> Result<Vec<String>, Error> {
		let mut info: Vec<String> = vec![];

		self.usart.write(build_request!(Request::Info))?;

		let mut data_count: [u8; 1] = [0;1];
		self.usart.read(&mut data_count)?;

		while data_count[0] > 0 {
			let mut len: [u8; 1] = [0;1];
			self.usart.read(&mut len)?;

			let mut data: Vec<u8> = vec![0; len[0] as usize];
			self.usart.read(data.as_mut_slice())?;

			info.push(
				data.into_iter()
					.map(|c| char::from(c))
					.collect::<String>()
			);

			sleep(Duration::from_millis(4));	// Add delay to give the board time to catch up with our requests			
			data_count[0] -= 1;
		}

		let mut aw: [u8; 1] = [0; 1];		
		self.usart.read(&mut aw)?;		

		if aw[0] != B15F::MSG_OK {
			return Err(format!("Board info is faulty: code {}", aw[0]).into());
		}		

		Ok(info)
	}

	/// Clears data in the USART buffers on this device and on the B15
	pub fn discard(&mut self) -> Result<(), Error> {
		self.usart.clear(serialport::ClearBuffer::Output)?;

		for _ in 0..16 {
			self.usart.write(build_request![Request::Discard])?;
			sleep(Duration::from_millis(4));
		}

		self.usart.clear(serialport::ClearBuffer::Input)?;

		Ok(())
	}

	/// Tests the connetion to the B15
	/// 
	/// To test the connection a `Request::Test` request will be sent
	/// to the board together with a randomly generated value. If the
	/// board returns that value the connection is working correctly.
	/// 
	/// ## Examples
	/// ```
	/// use b15f::B15F;
	/// 
	/// fn main() {
	/// 	let mut drv = B15F::new().unwrap();
	/// 	
	/// 	if let Err(err) = drv.test_connection() {
	/// 		panic!("Connection is not working: {err}");
	/// 	}
	/// }
	/// ```
	pub fn test_connection(&mut self) -> Result<(), Error> {
		let dummy: u8 = rand::thread_rng().gen_range(0x00..=0xFF);

		self.usart.write(build_request![Request::Test, dummy])?;
		
		let mut buffer: [u8; 2]= [0; 2];
		self.usart.read(&mut buffer)?;

		if buffer[0] != B15F::MSG_OK || buffer[1] != dummy {
			return Err("Test request failed".into());
		}

		Ok(())
	}

	#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
	fn get_devices() -> Vec<String> {
		let output = Command::new("bash")
			.args(["-c", "ls /dev/ttyAMA*"])
			.output()
			.expect("Failed to get serial interface");

		String::from_utf8(output.stdout)
			.expect("Failed to convert stdout to string")
			.split_ascii_whitespace()
			.map(|item| item.into())
			.collect()
	}

	#[cfg(not(target_arch = "arm"))]
	#[cfg(not(target_arch = "aarch64"))]
	fn get_devices() -> Vec<String> {
    

		let output = Command::new("bash")
			.args(["-c", "ls /dev/ttyUSB*"])
			.output()
			.expect("Failed to get serial interface");

		String::from_utf8(output.stdout)
			.expect("Failed to convert stdout to string")
			.split_ascii_whitespace()
			.map(|item| item.into())
			.collect()
	}
}

impl Debug for B15F {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "Baudrate:  {}", self.usart.baud_rate().unwrap())?;
		writeln!(f, "Data bits: {:?}", self.usart.data_bits().unwrap())?;
		writeln!(f, "Parity:    {:?}", self.usart.parity().unwrap())
	}
}