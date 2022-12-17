use b15f::B15F;

fn main() {
	let _drv = match B15F::new() {
		Ok(val) => val,
		Err(error) => {
			eprintln!("{error}");
			return;
		}
	};
}