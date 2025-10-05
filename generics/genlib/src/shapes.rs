use crate::point::{	Point};
pub trait Shape {
	fn area(&self) -> f64;
	fn perimeter(&self) -> f64;
	fn name(&self) -> &str {
		let name = std::any::type_name::<Self>();
		name.rsplit("::").next().unwrap_or("<Unknown Shape>")
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

impl Circle {
	pub fn new(center: Point, radius: f64) -> Circle {
		Circle { center, radius }
	}

}

impl Rectangle {
	pub fn new(tl: Point, br: Point) -> Rectangle {
		Rectangle { tl, br }
	}
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