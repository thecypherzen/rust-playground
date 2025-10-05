pub mod pair;
pub mod point;
pub mod shapes;
use std::fmt::Debug;

pub fn debug<T: Debug>(item: &T) {
		println!("{:?}", item);
}

pub fn describe(shape: &impl shapes::Shape) {
	let name = shape.name().rsplit("::").next().unwrap_or("<Unknown Shape>");
	println!("[{} - Area: {}, Perimeter: {}]", name , shape.area(), shape.perimeter());
}