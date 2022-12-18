use b15f::b15f::B15F;

fn main() -> Result<(), String> {
	let mut drv = B15F::new()?;

	println!("{}", drv.digital_read::<0>().unwrap());
	Ok(())
}