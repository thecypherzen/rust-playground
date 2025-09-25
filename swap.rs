#[allow(dead_code)]

use std::fmt::*;

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
fn swap_numbers(i: &mut i32, j: &mut i32) {
	*i = *i ^ *j;
	*j = *i ^ *j;
	*i = *i ^ *j;
}

fn main() {
	let pair = Pair(3, 420);
	println!("--- Swapping a Tuple Struct ---");
	println!("Original:{} Swapped:{}", pair, pair.swap());
	println!("--- Swapping Two Variables ---");

	let mut a = 13;
	let mut b = -10;
	println!("(before) => a: {} b: {}", a, b);
	swap_numbers(&mut a, &mut b);
	println!("(after) => a: {} b: {}", a, b);
}
