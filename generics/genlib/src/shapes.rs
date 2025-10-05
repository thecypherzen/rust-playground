use crate::point::{	Point};
pub trait Shape {
	fn area(&self) -> f64;
	fn perimeter(&self) -> f64;
	fn name(&self) -> &str {
		std::any::type_name::<Self>()
	}
}

#[derive(Debug)]
pub struct Rectangle {
	pub tl: Point,
	pub br: Point
}

#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64
}

impl Shape for Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * self.radius * self.radius
	}

	fn perimeter(&self) -> f64 {
		std::f64::consts::PI * self.radius * 2.0
	}
}

impl Shape for Rectangle{
	fn area(&self) -> f64 {
		let width = self.br.x - self.tl.x;
		let height = self.br.y - self.tl.y;
		width * height
	}
	fn perimeter(&self) -> f64 {
		let width = self.br.x - self.tl.x;
		let height = self.br.y - self.tl.y;
		2.0 * (width + height)
	}
}