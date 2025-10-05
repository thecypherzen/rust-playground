pub trait PointTrait: Into<f64> {}

#[derive(Debug, Copy, Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64
}

impl Point {
	pub fn new(x:f64, y: f64) -> Point {
		Point { x,  y }
	}

	pub fn from<T: PointTrait>(x: T, y: T) -> Point {
		Point { x: x.into(), y: y.into() }
	}
}