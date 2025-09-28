use std::fmt::Debug;
// recieves an array by ref and prints it
fn print_array_by_ref<T>(array: &T)
	where T: Debug {
	println!("[PABR] {:?}", array);
}

fn print_vector<T>(vector: &Vec<T>)
	where T: Debug {
	println!("[PVECT] {:?}", vector);
}

fn print_slice<T>(slice: &[T])
 where T: Debug {
		println!("[PSLC] {:?}", slice);
 }

 fn box_mult_ret_own<T>(v: Box<T>) -> Box<T>
  where T: std::ops::Mul<Output = T> + Debug + num_traits::FromPrimitive {
		println!("[MRETOWN] multiplying {:?} by 3", v);
		Box::new(*v * T::from_i32(3).unwrap())
	}

fn main() {
	let arr1: [i32; 13] = [19; 13];
	let arr2: [&str; 6] = ["Hello", " there.", " Forever", " on", " we", " go!"];

	let v1: Vec<String> = vec!["Hello".to_string(), " there".to_string(), format!("{}", "people!")];
	let v2: Vec<&str> = vec!["Hello", " there.", " Forever", " on", " we", " go!"];

	let n1 = Box::new(123);
	let n2 = Box::new(-92.105);

	// printings
	println!("------- [Whole Arrays/Vectors] -------");
	print_array_by_ref(&arr1);
	print_array_by_ref(&arr2);
	print_vector(&v1);
	print_vector(&v2);
	println!("\n------- [Arrays/Vector Slices] -------");
	print_slice(&arr1[1 .. 4]);
	print_slice(&arr2[3..]);
	print_slice(&v1[..1]);
	println!("\n------- Return ownership -------");
	let n1 = box_mult_ret_own(n1);
	println!("\tnew value: {}", n1);
	let n2 = box_mult_ret_own(n2);
	println!("\tnew value: {}", n2);
}
