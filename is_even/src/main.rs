/*
 * Creates a function to check if a value is even or not
 * The function is designed to accept both integers and floats of various sizes.
 */
use std::ops::Rem;
use std::cmp::PartialEq;
use num_traits::FromPrimitive;

fn is_even<T>(num: T) -> bool
where T: Copy + Rem<Output=T> + FromPrimitive + PartialEq<T>{
	let div: T = T::from_i32(2).unwrap();
	let zero: T = T::from_i32(0).unwrap();
	num % div == zero
}

fn main() {
	let n1 = 24;
	let n2 = 18.00;
	let n3 = 19u32;
	let n4 = 52u64;
	let n5 = 3i128;
	let n6 = 6f64;
	println!("{} is even ? {}", n1, is_even(n1));
	println!("{} is even ? {}", n2, is_even(n2));
	println!("{} is even ? {}", n3, is_even(n3));
	println!("{} is even ? {}", n4, is_even(n4));
	println!("{} is even ? {}", n5, is_even(n5));
	println!("{} is even ? {}", n6, is_even(n6));
}