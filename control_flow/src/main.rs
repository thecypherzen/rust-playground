use std::io::Write;


// sum numbers from 1 - 100
fn sum_to_100_for() -> i32 {
	let mut sum = 0;
	for i in 1..=100 {
		sum += i;
	}
	sum
}

fn sum_to_100_loop() -> i32 {
	let mut sum = 0;
	let mut i = 1;
	loop {
		if i == 100 {
			break;
		}
		sum += i;
		i += 1;
	}
	sum
}

// get random number
fn get_random_integer(i: i32, j: i32) -> i32 {
	rand::random_range(i..=j)
}

fn main() {
	println!("------ COUNTING TO 100 -------");
  println!("Sum from 1 - 100 (for)= {}", sum_to_100_for());
  println!("Sum from 1 - 100 (loop)= {}", sum_to_100_loop());

	println!("\n------ NUMBER GUESSER -------\n");
	let num_range = (1, 50);
	println!("Guess the right number in 5 tries HINT: {}-{}", num_range.0, num_range.1);
	let mut user_input = String::new();
	let random_no = get_random_integer(num_range.0, num_range.1);
	let mut max_retries = 5;

	while max_retries > 0 {
		user_input.clear();
		print!("Enter your guess: {} ", if max_retries == 5 { String::new() } else { format!(" ({}) retries left", max_retries) });
		std::io::stdout().flush().expect("Failed to flush stdout");
		std::io::stdin().read_line(&mut user_input).expect("Failed to read input)");
		let guessed_no: i32 = match  user_input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue
		};
	  if guessed_no < random_no {
			println!(" Opps too small!");
			max_retries -= 1;
			continue;
		} else if guessed_no > random_no {
			println!(" Oops to big!");
			max_retries -= 1;
			continue;
		} else {
			println!(" You win!");
			break;
		}
	}
	if max_retries == 0 {
		println!("You lose! The number was {}", random_no);
	}
}
