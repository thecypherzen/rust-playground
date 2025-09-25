#[allow(dead_code)]

use std::fmt::*;
use std::ops::BitXor;

struct Pair(i32, i32);

// Implement display
impl Display for Pair {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({} {})", self.0, self.1)
	}
}

// Implement swap on instance
impl Pair {
	fn swap(&self) -> Pair {
		Pair(self.1, self.0)
	}
}

// Implement a generic swapping function
fn swap_integers<T>(i: &mut T, j: &mut T)
where T: BitXor<Output=T> + Copy {
	*i = *i ^ *j;
	*j = *i ^ *j;
	*i = *i ^ *j;
}

fn swap<T>(i: &mut T, j: &mut T) {
	std::mem::swap(i, j)
}

fn main() {
	let pair = Pair(3, 420);
	println!("--- Swapping a Tuple Struct ---");
	println!("Original:{} Swapped:{}", pair, pair.swap());
	println!("\n--- Swapping Two Integers ---");

	let mut a = 14;
	let mut b = -12;
	println!("(before) => int1: {} int2: {}", a, b);
	swap_integers(&mut a, &mut b);
	println!("(after) => int1: {} int2: {}", a, b);
	println!("\n--- Swapping Two Floats ---");

	let mut c = 14.33;
	let mut d = 1.24;
	println!("(before) => f1: {} f2: {}", c, d);
	swap(&mut c, &mut d);
	println!("(after) => f1: {} f2: {}", c, d);
}
