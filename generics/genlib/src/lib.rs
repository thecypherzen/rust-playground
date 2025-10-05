pub mod pair;
pub mod point;
pub mod shapes;
use std::fmt::Debug;

pub fn debug<T: Debug>(item: &T) {
		println!("{:?}", item);
}

pub fn describe(shape: &impl shapes::Shape) {
	println!("[{} - Area: {}, Perimeter: {}]", shape.name() , shape.area(), shape.perimeter());
}