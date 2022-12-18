#![feature(test)]

extern crate test;

mod tests {
	use super::*;
	use test::Bencher;
	use b15f::B15F;

	#[bench]
	fn bench_create_instance(b: &mut Bencher) {
		b.iter(|| B15F::new());
	}

	#[bench]
	fn bench_digital_write(b: &mut Bencher) {
		let mut drv = B15F::new().unwrap();
		
		b.iter(move || drv.digital_write::<0>(0xAB).unwrap());
	}

	#[bench]
	fn bench_digital_read(b: &mut Bencher) {
		let mut drv = B15F::new().unwrap();

		b.iter(move || drv.digital_read::<0>().unwrap());
	}
}