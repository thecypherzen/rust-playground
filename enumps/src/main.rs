#![allow(dead_code)]

use crate::Direction::*;
use std::fmt::{Display, Formatter, Result};

// Direction enum
enum Direction {
	North, South, East, West, Move(i32, i32)
}

// Option like enum
enum OptionLike<T> {
	Some(T),
	None
}

impl Display for Direction {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", match *self {
			North => String::from("↑ North"),
			South => String::from("↓ South"),
			East => String::from("→ East"),
			West => String::from("← West"),
			Move(x, y) => {
				let r = format!("Moving ({}, {})", x, y);
				r
			}
		})
	}
}

fn empty_vector<T: std::fmt::Debug>(vect: &mut Vec<T>) {
	println!("Initial vect length: {}", vect.len());
	while let Some(k) = vect.pop() {
		println!("removing: {:?} | L: ({})", k, vect.len());
	}
}

fn main() {
    let dir = North;
		let mv = Move(24, 333);
		println!("Im heading {}.", dir);
		println!("Im {}.", mv);
		let opt_like = OptionLike::Some(4i64);
		if let OptionLike::Some(k) = opt_like {
			println!("{} is OptionLike::Some value", k)
		} else {println!("It's most likely a None")
	};
	let opt_like_2 = OptionLike::<i32>::None;
		if let OptionLike::Some(k) = opt_like_2 {
			println!("{} is OptionLike::Some value", k)
		} else {println!("It's a None")
	};

	let mut v1 = vec![10, 2];
	empty_vector(&mut v1);
}
