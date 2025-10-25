struct AssertInRange<const VALUE: usize, const MIN: usize, const MAX: usize>;

impl<const VALUE: usize, const MIN: usize, const MAX: usize> AssertInRange<VALUE, MIN, MAX> {
	const OK: () = assert!(MIN <= VALUE && VALUE <= MAX);
}

pub fn assert_in_range<const VALUE: usize, const MIN: usize, const MAX: usize> () {
	let () = AssertInRange::<VALUE, MIN, MAX>::OK;
}