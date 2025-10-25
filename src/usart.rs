use serialport::SerialPort;
use crate::error::Error;

pub fn read_sized<const N: usize> (usart: &mut Box<dyn SerialPort>) -> Result<[u8; N], Error> {
	let mut buf: [u8; N] = [0; N];
	
	usart.read(&mut buf)?;
	Ok(buf)
}