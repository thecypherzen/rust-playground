fn to_precision(num: f64, prec: i32) -> f64 {
		let factor = 10f64.powi(prec);
		(num * factor).round() / factor
}

fn main() {
	let n1 = 13.923723f64;
	let p1 = 2i32;
	println!("{} to {}dps = {}", n1, p1, to_precision(n1, p1));
}