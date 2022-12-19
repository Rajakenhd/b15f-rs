use b15f::b15f::B15F;

fn main() -> Result<(), String> {
	let mut drv = B15F::new()?;

	println!("BA-0: {:b}", drv.digital_read::<0>()?);
	println!("BA-1: {:b}", drv.digital_read::<1>()?);
	println!("DIP : {:b}", drv.read_dip_switch()?);

	Ok(())
}