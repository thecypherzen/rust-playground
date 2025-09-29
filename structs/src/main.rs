// a struct with x & y props
#![allow(dead_code)]

/**
 * to_precision: a function that rounds a float to specified
 * precision
 */
fn to_precision<T: Into<f64>>(n:T, p: u32) -> f64 {
	let factor = 10i32.pow(p) as f64;
	(n.into() * factor).round() / factor
}

// define Point struct
#[derive(Debug)]
struct Point {
	x: f64, y: f64
}

// implement a Point constructor
impl Point {
	fn new<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
		Point { x: x.into(), y: y.into()}
	}
	/**
	 * Calculates the distance from origin of a point
	 */
	fn distance_from_origin(&self) -> f64 {
		to_precision((self.x.powi(2)+ self.y.powi(2)).sqrt(), 2)
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
		let points = point!({x: 22, y:2}, {y: 1.2f64, x: 13.09f64}, {x: 19.0f32, y: 22f32}, {x: 123.92f64, y: 55.55f32});
		let [first, rest @..] = points.as_slice() else { todo!() };
		println!("Point1: {:?}\nRest: {:?}", first, rest);
		println!("Point1 distance from origin: {}", first.distance_from_origin());
}
