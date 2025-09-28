// Takes ownership of a string and prints it
fn own_str_print(string: String) {
	println!("[owned-str-print] {}", string);
}

// borrows a string immutably
fn borrow_string(string: &String) {
	println!("[borrow_string] {}", string);
}

// append text a string
fn append_str(chunk: &str, string: &mut String) {
	string.push_str(chunk);
	println!("[append_str] {}", string);
}

fn get_str_info(s: &str) -> (usize, char) {
	let r1 = s.chars().next().unwrap();
	let r2 = s.chars().nth(1).unwrap();
	println!("c1: {}, c2: {}", r1, r2);
	(s.len(), s.chars().next().unwrap())
}

fn main() {
	let s1: String = String::from("Owned string");
	let s2: String = String::from("I was borrowed immutaby");
	let mut s3: String = String::from("This was just added:");
	let added_str: &str = " BABY STRING";
	own_str_print(s1);
	borrow_string(&s2);
	append_str(added_str, &mut s3);

	println!("\n ------- STRING INFO ------- ");
	let str1 = "String one";
	let str2 = "This Is String Two";
	let str1_info = get_str_info(str1);
	let str2_info = get_str_info(str2);
	println!("String 1:[{}] info: {:?}", str1, str1_info);
	println!("String 2:[{}] info: {:?},", str2, str2_info);
}
