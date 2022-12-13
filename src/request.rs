//! This module contains the request data used to communicate
//! with the B15 via USART. 
//! 
//! Using a direct USART connection to the B15 is discouraged,
//! if you are trying to interact with the B15 consider using 
//! the `b15f::B15F` structure instead.

#[repr(u8)]
enum RequestType {
	Discard = 0
}

pub struct Request {
	req: Vec<u8>
}

impl Request {
	pub fn new() -> Request {
		Request { req: vec![] }
	}

	pub fn discard(mut self) -> Self {
		self.req.push(RequestType::Discard as u8);
		self
	}

	pub fn done(self) -> Vec<u8> {
		self.req
	}
}