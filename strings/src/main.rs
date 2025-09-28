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

fn main() {
	let s1: String = String::from("Owned string");
	let s2: String = String::from("I was borrowed immutaby");
	let mut s3: String = String::from("This was just added:");
	let added_str: &str = " BABY STRING";
	own_str_print(s1);
	borrow_string(&s2);
	append_str(added_str, &mut s3);
}
