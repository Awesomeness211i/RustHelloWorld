use std::io;

pub fn getinput(string: &mut String) {
	io::stdin().read_line(string).expect("Failed to read line");
}