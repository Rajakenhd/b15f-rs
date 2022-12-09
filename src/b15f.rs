//! This module contains all the structures and functions related to
//! interacting with the B15 on a high level. If you are writing code
//! for the B15, this is the module you want to use.

use std::process::Command;

/// Hardcoded commit hash of the most recent firmware
static COMMIT_HASH: &'static str = "bc459c80cec755d7df2c11a807d74e085cbed332";

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
#[derive(Debug)]
pub struct B15F {
	
}

impl B15F {
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
	pub fn new() -> Result<B15F, &'static str> {
		let drv = B15F {};
		drv.init()
	}

	fn init(self) -> Result<B15F, &'static str> {
		let devices = B15F::get_devices();

		let device = match devices.first() {
			Some(item) => item,
			None => return Err("Failed to find adapter")
		};

		log!("Using adapter: {}", device);

		log_start!("Establish connection with adapter");
		todo!("Implement USART");
		log_end!("Ok!");

		log_start!("Testing connection");
		todo!("Test connection");
		log_end!("Ok!");

		let info = self.get_board_info();
		log!("AVR firmware version: {} built at {} ({})", info[0], info[1], info[2]);

		let avr_commit_hash = info[3];
		if avr_commit_hash != COMMIT_HASH {
			log!("Different commit hashes: {} vs {}", avr_commit_hash, COMMIT_HASH);
			return Err("Versions incompatible. Please update the software!");
		}

		Ok(self)
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
	/// let drv = B15F::new().unwrap();
	/// 
	/// // Print each bit of information on a new line
	/// drv.get_board_info()
	/// 	.iter()
	/// 	.for_each(|info| println!("{info}"));
	/// ```
	pub fn get_board_info(&self) -> Vec<&str> {
		todo!();
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