#![allow(dead_code)]

use crate::Direction::*;
use std::fmt::{Display, Formatter, Result};
use num_traits::{FromPrimitive};
use std::cmp::PartialEq;
use std::ops::Div;

// Direction enum
enum Direction {
	North, South, East, West, Move(i32, i32)
}

#[derive(Debug)]
// ResultLike enum
enum ResultLike<T, E> {
	Ok(T),
	Err(E)
}

// Option like enum
enum OptionLike<T> {
	Some(T),
	None
}

enum Errors {
	DivisionByZero
}

// display for OptionLike
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

fn divide<T:>(x:T, y:T) -> ResultLike<f64, Errors>
where T: PartialEq<T> + FromPrimitive + Into<f64> + Div {
	if y == T::from_i32(0).unwrap() {
		ResultLike::Err(Errors::DivisionByZero)
	} else {
		let res = x.into() / y.into();
		ResultLike::Ok(res)
	}
}

/**
 * Linked List
 */
#[derive(Debug)]
enum List<T: Display + Copy> {
	Node(T, Box<List<T>>),
	Nil
}

impl<T: Display + Copy> List<T> {
	fn new() -> List<T> {
		List::Nil
	}

	fn prepend(self, elem: T) -> List<T> {
		List::Node(elem, Box::new(self))
	}

	fn stringify(&self) -> String {
		match self {
			List::Node(h, t) => format!("{}{}{}", t.stringify(), if t.len() > 0 {", "} else {""}, h),
			List::Nil => String::new()
		}
	}

	fn len(&self) -> u32 {
		match self {
			List::Node(_, rest) => 1u32 + rest.len(),
			List::Nil => 0u32
		}
	}

	fn print(&self) {
		println!("{}", self);
	}
}


impl<T: Display + Copy> Display for List<T> {
	fn fmt(&self, f: &mut Formatter) -> Result {
			write!(f, "[{}]", self.stringify())
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
	let nums = ((3, 4), (9.0, -3.0), (13, 0));
	if let ResultLike::Ok(res2) = divide((nums.0).0, (nums.0).1) {
		println!("{}/{} = {}", (nums.0).0, (nums.0).1, res2);
	} else {
		println!("{}/{} = Error", (nums.0).0, (nums.0).1);
	}

	let div2 =  divide((nums.1).0, (nums.1).1);
	match div2 {
		ResultLike::Ok(res2) => println!("{}/{} = {}", (nums.1).0, (nums.1).1, res2),
		_ => println!("{}/{} = Error", (nums.1).0, (nums.1).1)
	}
	let div3 = divide((nums.2).0, (nums.2).1);
	match div3 {
		ResultLike::Ok(res3) => println!("{}/{} = {}", (nums.2).0, (nums.2).1, res3),
		ResultLike::Err(Errors::DivisionByZero) => println!("{}/{} = Error", (nums.2).0, (nums.2).1)
	}

	println!("\n------ LINKED LIST --------");
	let list_1 = List::new().prepend(2.0).prepend(2.5).prepend(3.0);
	println!("List: {}", list_1);
	list_1.print();
}
