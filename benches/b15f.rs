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
}