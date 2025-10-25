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
	Discard 		= 0,
	Test 			= 1,
	Info 			= 2,
	IntTest			= 3,
	SelfTest		= 4,
	DigitalWrite0 	= 5,
	DigitalWrite1 	= 6,
	DigitalRead0	= 7,
	DigitalRead1	= 8,
	ReadDipSwitch	= 9,
	AnalogWrite0	= 10,
	AnalogWrite1	= 11,
	AnalogRead		= 12,
	AdcDacStroke	= 13,
	PwmSetFreq		= 14,
	PwmSetValue		= 15,
	SetMem8			= 16,
	GetMem8			= 17,
	GetMem16		= 18,
	SetMem16		= 19,
	CounterOffset 	= 20,
	ServoEnable		= 21,
	ServoDisable	= 22,
	ServoSetPos		= 23,
}