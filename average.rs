#[allow(dead_code)]

// Using Slice
fn average(nums: &[f64]) -> Result<f64, &'static str> {
	if nums.is_empty() {
		return Err("empty set");
	}
	let sum: f64 = nums.iter().sum();
	Ok(sum / nums.len() as f64)
}


// Using a slice of generic Type
fn free_average<T: Into<f64> + Copy>(nums: &[T] ) -> Result<f64, &'static str> {
	let sum: f64 = nums.iter().map(|n| (*n).into()).sum();
	Ok(sum / nums.len() as f64)
}


// Using macro
macro_rules! average {
	($($x:expr),+) => {{
		let nums = vec![$($x.into()),+];
		if (nums.is_empty()) {
			Err("Error")
		} else {
			let sum: f64 = nums.iter().sum();
			Ok( sum / nums.len() as f64)
		}
	}}
}
fn main() {
	let all_float = [231.0, 12.1, 19.0, 51.2, 45.0];
	let mixed_nums = (231, 12.1, 19, 51.2, 45);
	let all_float_av = free_average(&all_float);
	let mixed_nums_av = average!(231, 12.1, 19, 51.2, 45);
	println!("{:?} | Av => {:?}", all_float, all_float_av);
	println!("{:?} | Av => {:?}", mixed_nums, mixed_nums_av);
}