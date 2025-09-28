// a struct with x & y props
//struct Point {
//	x: i32, y: i32
//}

macro_rules! point {
	() => {};
	(
		{x: $x:expr, y: $y:expr}$(,$($rest:tt)*)?
	) => {
		println!("x: {}, y: {}", $x, $y);
		$( point!($($rest)*) )*
	};
	(
		{y: $y:expr, x: $x:expr}$(,$($rest:tt)*)?
	) => {
		println!("x: {}, y: {}", $x, $y);
		$( point!($($rest)*) )*
	};
}

fn main() {
		point!({x: 2.2, y:2.0}, {y: 14, x: 13}, {x: 19.0, y: 22.9}, {x: 123.92, y: 55.550});
}
