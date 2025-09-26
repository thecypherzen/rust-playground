// A collection of factorial approaches

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use num_bigint::BigUint;
use num_traits::identities::One;

enum Error {
	OverflowError
}
/**
 * A factorial function that operates only on u128 values
 * Uses a hashmap for memoization
 * Gracefully handles overflowing errors without breaking
 */
fn factorial_limited(n: u128, memo: &mut HashMap<u128, u128>) -> Result<u128, Error> {
	// check for value in cache
	if let Some(fact) = memo.get(&n) {
		return Ok(*fact);
	}

	// get new factorial
	let res = if n <= 1u128 {
		Ok(1u128)
	} else {
		if let Some(next) = n.checked_mul(factorial_limited(n-1, memo)?) {
			Ok(next)
		} else {
			Err(Error::OverflowError)
		}
	};
	// save in cache if successful and return
	if let Ok(t) = res {
		memo.insert(n, t);
	}
	res
}



/**
 * Global Factorials Cache. Has to be global for efficiency
 * Mutex is used for thread safety - even though it's not needed in this
 * context, it's a good concept to adopt
 */
 static CACHE: Lazy<Mutex<HashMap<u128, BigUint>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/**
 * A factorial function that is built to handle very very large numbers, even
 * up 500!
 * It uses the BigUint module and a global HashMap. The global HashMap ensures
 * that the cache persists between calls, throughout the life of the program
 */
 fn factorial (n: u128) -> Result<BigUint, Error> {
		// check cache
		if let Some(fact) = CACHE.lock().unwrap().get(&n) {
			return Ok(fact.clone());
		};
		// calculate new factorial
		let fact = if n <= 1u128 {
			Ok(BigUint::one())
		} else {
			Ok(BigUint::from(n) * factorial(n - 1)?)
		};

		if let Ok(ref t) = fact {
			CACHE.lock().unwrap().insert(n, t.clone());
		};

		fact
 }



fn main() {
	println!("=== OVERFLOWING FACTORIAL ===");
	let mut memo = HashMap::new();
	let n1 = 5u128;
	match factorial_limited(n1, &mut memo) {
		Ok(fact) => println!("{}! = {}", n1, fact),
		Err(Error::OverflowError) => println!("Overflow Error occured")
	}
	let n2 = 25u128;
	match factorial_limited(n2, &mut memo) {
		Ok(fact) => println!("{}! = {}", n2, fact),
		Err(Error::OverflowError) => println!("Overflow Error occured")
	}
	let n3 = 34u128;
	match factorial_limited(n3, &mut memo) {
		Ok(fact) => println!("{}! = {}", n3, fact),
		Err(Error::OverflowError) => println!("{}! = Overflow Error occured", n3)
	}
	let n4 = 50u128;
	match factorial_limited(n4, &mut memo) {
		Ok(fact) => println!("{}! = {}", n3, fact),
		Err(Error::OverflowError) => println!("{}! = Overflow Error occured", n4)
	}


	println!("\n=== NON-OVERFLOWING FACTORIAL===");
	let n5 = 50u128;
	match factorial(n5) {
		Ok(fact) => println!("{}! = {}", n5, fact),
		_ => println!("{}! = An Error occured", n5)
	}
	let n6 = 100u128;
	match factorial(n6) {
		Ok(fact) => println!("{}! = {}", n6, fact),
		_ => println!("{}! = An Error occured", n6)
	}
	let n7 = 300u128;
	match factorial(n7) {
		Ok(fact) => println!("{}! = {}", n7, fact),
		_ => println!("{}! = An Error occured", n7)
	}
	let n8 = 1000u128;
	match factorial(n8) {
		Ok(fact) => println!("{}! = {}", n8, fact),
		_ => println!("{}! = An Error occured", n8)
	}
}
