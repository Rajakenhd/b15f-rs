//! This module contains the request data used to communicate
//! with the B15 via USART. 
//! 
//! Using a direct USART connection to the B15 is discouraged,
//! if you are trying to interact with the B15 consider using 
//! the `b15f::B15F` structure instead.

// TODO: There should be a more elegant way to do this

#[macro_export]
/// Builds a new request buffer from the given data
macro_rules! build_request {
	[$($x:expr),*] => (
		&[$($x as u8),*]
	);
}

#[repr(u8)]
pub enum Request {
	Discard = 0,
	Test 	= 1,
	Info 	= 2
}