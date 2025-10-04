// a fizzbuzz program for any number of user's choice
use std::io::Write;
// implements the fizzbuzz function
fn fizzbuzz_to(n: u128) {
	for i in 1..=n {
		let (mod3, mod5) = (i % 3, i % 5);
		if mod3 == 0 && mod5 == 0 {
			println!("FizzBuzz")
		} else if mod3 == 0 {
			println!("Fizz");
		} else if mod5 == 0 {
			println!("Buzz");
		} else {
			println!("{}", i);
		}
	}
}

fn main() {
	let mut entry = String::new();
	entry.clear();
	print!("Do fizzbuzz from 1 - ? : ");
	std::io::stdout().flush().expect("Failed to flush stdout");
	std::io::stdin().read_line(&mut entry).expect("Reading failed");
	let num: u128 = entry.trim().parse().expect("Failed to parse  from stdout");
	fizzbuzz_to(num);
}
