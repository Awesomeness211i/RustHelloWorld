use std::io;

pub fn getconsoleinput(string: &mut String) {
	io::stdin().read_line(string).expect("Failed to read line");
}