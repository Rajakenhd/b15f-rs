use std::{thread::sleep, time::Duration};

use b15f::b15f::B15F;

fn main() {
	let mut drv = B15F::new().unwrap();

	let mut position = 0;
	let mut direction = 1;

	loop {
		drv.digital_write::<0>(1 << position);

		position += direction;
		if position >= 7 || position <= 0 {
			direction *= -1;
		}

		sleep(Duration::from_millis(40));
	}
}