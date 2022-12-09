use std::error::Error;

use b15f::B15F;

fn main() -> Result<(), Box<dyn Error>>{
	let drv = B15F::new()?;

	Ok(())
}