// a struct with x & y props
#[allow(dead_code)]
#[derive(Debug)]
struct Point {
	x: f64, y: f64
}

impl Point {
	fn new<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Self {
		Point { x: x.into(), y: y.into()}
	}
}

macro_rules! point {
	() => {Vec::<Point>::new()};
	(
		{x: $x:expr, y: $y:expr}$(,$($rest:tt)*)?
	) => {{
		let mut vect = Vec::<Point>::new();
		vect.push(Point::new($x, $y));
		vect.extend(point!($($($rest)*)?));
		vect
	}};
	(
		{y: $y:expr, x: $x:expr}$(,$($rest:tt)*)?
	) => {{
		let mut v2 = Vec::<Point>::new();
		v2.push(Point::new($x, $y));
		v2.extend(point!($($($rest)*)?));
		v2
	}};
}

fn main() {
		let r = point!({x: 22, y:2}, {y: 1.2f64, x: 13.09f64}, {x: 19.0f32, y: 22f32}, {x: 123.92f64, y: 55.55f32});
		println!("Points::{:?}", r);
}
