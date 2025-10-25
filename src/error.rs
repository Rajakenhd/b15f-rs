use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
	message: String
}

impl Error {
	pub fn new(what: &str) -> Error {
		Error { message: String::from(what) }
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error { message: err.to_string() }	
	}
}

impl From<serialport::Error> for Error {
	fn from(err: serialport::Error) -> Self {
		Error { message: err.to_string() }	
	}	
}

impl From<&str> for Error {
	fn from(msg: &str) -> Self {
		Error::new(msg)	
	}
}

impl From<String> for Error {
	fn from(msg: String) -> Self {
		Error { message: msg }
	}
}

impl From<Error> for String {
	fn from(err: Error) -> Self {
		err.message	
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}