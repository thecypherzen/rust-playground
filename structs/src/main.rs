#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};

/**
 * to_precision: a function that rounds a float to specified
 * precision
 */
fn to_precision<T: Into<f64>>(n:T, p:u32) -> f64 {
	let factor = 10i32.pow(p) as f64;
	(n.into() * factor).round() / factor
}
// a struct with x & y props
// define Point struct
#[derive(Debug, Clone, Copy)]
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

	fn distance_from(&self, point: Point) -> f64 {
		to_precision((((point.x - self.x).powi(2)) + ((point.y - self.y).powi(2))).sqrt(), 2)
	}
}

// display
impl Display for Point {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "P({}, {})", self.x, self.y)
	}
}

/**
 * Rectangle Struct
 */
#[derive(Debug, Clone)]
struct Rectangle {
	tl: Point,
	br: Point
}
impl Display for Rectangle {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "R{{tl: {}, br: {}}}", self.tl, self.br)
	}
}
impl Rectangle {
	// constructor
	fn new(tl: Point, br: Point) -> Rectangle {
		Rectangle {tl, br}
	}

	// calculates area of rectangle
	fn area(&self, dps:Option<u32>) -> f64 {
		let presc = dps.unwrap_or(2);
		to_precision(self.length() * self.width(), presc)
	}

	fn breadth(&self) -> f64 {
		self.width()
	}

	fn point_is_within(&self, p: Point) -> bool {
		let (xmin, xmax) = (self.tl.x.min(self.br.x), self.tl.x.max(self.br.x));
		let (ymin, ymax) = (self.tl.y.min(self.br.y), self.tl.y.max(self.br.y));
		p.x >= xmin && p.x <= xmax && p.y >= ymin && p.y <= ymax
	}

	fn length(&self) -> f64 {
		self.br.distance_from(Point::new(self.tl.x, self.br.y))
	}

	fn width(&self) -> f64 {
		self.tl.distance_from(Point::new(self.tl.x, self.br.y))
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
		let [p1, p2, rest @..] = points.as_slice() else { todo!() };
		let rect1 = Rectangle::new(*p1, *p2);
		let rect2 = Rectangle::new(rest[0], rest[1]);
		println!("Rectangle: {}, Area = L: {} x B: {} = {}", rect1, rect1.length(), rect1.breadth(), rect1.area(Some(3)));
		println!("Rectangle: {}, Area = L: {} x B: {} = {}", rect2, rect2.length(), rect2.breadth(), rect2.area(None));
		println!("{} is within {} ? {}", Point {x:15f64, y:1.5f64}, rect1, rect1.point_is_within(Point { x: 15.0f64, y: 1.5f64}));
		println!("{} is within {} ? {}", Point {x: 12f64, y: 3f64}, rect1, rect1.point_is_within(Point { x: 3f64, y: 2f64}));
}
