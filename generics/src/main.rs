use std::cmp::{PartialOrd};

fn is_larger<T: PartialOrd>(x: T, y: T) -> T {
	if x > y {
		return x;
	}
	y
}

fn main() {
		println!("----------- IS GREATER -----------");
    let cmp = [(2, 4), (-13, -1)];
		let cmp2 = [(2.0, -2.0), (3.4, 9.9)];
		let mut i = 0;
		while let Some(c) = cmp.get(i) {
			i += 1;
			println!("{} vs {}:\t✔︎ {} is larger", c.0, c.1, is_larger(c.0, c.1));
		}
		i = 0;
		while let Some(c) = cmp2.get(i) {
			i += 1;
			println!("{} vs {}:\t✔︎ {} is larger", c.0, c.1, is_larger(c.0, c.1));
		}
}
