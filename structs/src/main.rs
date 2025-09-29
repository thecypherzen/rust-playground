// a struct with x & y props
#![allow(dead_code)]

fn to_precision<T: Into<f64>>(n:T, p: u32) -> f64 {
	let factor = 10i32.pow(p) as f64;
	(n.into() * factor).round() / factor
}

#[derive(Debug)]
struct Point {
	x: f64, y: f64
}

impl Point {
	fn new<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
		Point { x: x.into(), y: y.into()}
	}
}

// Point Generation Macro
macro_rules! point {
	() => { Vec::<Point>::new() };
	({x: $x:expr, y: $y:expr}$(, $($rest:tt)*)?) => {{
		let mut v = Vec::<Point>::new();
		v.push(Point::new(to_precision($x, 2), to_precision($y, 2)));
		v.extend(point!($($($rest)*)?));
		v
	}};
	({y: $y:expr, x: $x:expr}$(, $($rest:tt)*)?) => {{
		let mut v = Vec::<Point>::new();
		v.push(Point::new(to_precision($x, 2), to_precision($y, 2)));
		v.extend(point!($($($rest)*)?));
		v
	}};
}

fn main() {
		let r = point!({x: 22, y:2}, {y: 1.2f64, x: 13.09f64}, {x: 19.0f32, y: 22f32}, {x: 123.92f64, y: 55.55f32});
		println!("Points::{:?}", r);
}
