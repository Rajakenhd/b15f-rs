use serialport::SerialPort;
use crate::error::Error;

/// Reads from a USART connecction and casts the result into the specified type
/// 
/// # Errors
/// This macro may throw an `error::Error` when reading from the connection fails.
/// Due to the implementation of this macro, an erroneous result is returned early
/// with the `?` operator, and not unwrapped.
/// 
/// # Unsafe
/// This macro makes use of `std::mem::transmute`.
#[macro_export]
macro_rules! read_typed {
	($usart: expr, $T: ty) => {
		unsafe {
			std::mem::transmute(read_sized::<{ std::mem::size_of::<$T>() }>(&mut $usart)?)
		}
	};
}

pub fn read_sized<const N: usize> (usart: &mut Box<dyn SerialPort>) -> Result<[u8; N], Error> {
	let mut buf: [u8; N] = [0; N];
	
	usart.read(&mut buf)?;
	Ok(buf)
}