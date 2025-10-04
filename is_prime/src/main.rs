use std::io::Write;

fn is_prime(num: u64) -> bool {
	if num == 2 {
		return true;
	}
	if num < 2u64 || num % 2u64 == 0 {
		return false;
	}
	let number = num as f64;
	let sqrt = number.sqrt();
	let mut i = 3f64;
	while i <= sqrt {
		if number % i == 0f64 {
			return false;
		}
		i += 2f64;
	}
	true
}

fn main() {
	let mut val = String::new();
	loop {
		print!(" -> Check is Prime for: ");
		val.clear();
		std::io::stdout().flush().expect("failed to flush stdout");
		std::io::stdin().read_line(&mut val).expect("Reading number failed");
		// convert value to number
		let num: i64 = val.trim().parse().unwrap();
		let res = is_prime(num.unsigned_abs());
		let suffix = format!("is{}", match res {
			true => String::new(),
			false => String::from(" not")
		});
		println!("    {} {} prime.", num, suffix);
	}
}
