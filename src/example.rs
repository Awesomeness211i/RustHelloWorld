pub fn fibbonachi(n:u32) -> u32 {
	let mut fprev = 0;
	let mut fcurr = 1;
	if n == 0 {
		return fprev;
	}
	else if n == 1 {
		return fcurr;
	}
	for _i in 1..n {
		let fnext = fprev + fcurr;
		fprev = fcurr;
		fcurr = fnext;
	}
	return fcurr;
	//recursion
	return fibbonachi(n-1) +  fibbonachi(n - 2);
}

//Function with arguments
pub fn answer(x: i32, y: i32) -> i32 {
	return x * y + 5;
}

#[allow(dead_code)]
pub enum Quest {
	Long,
	Medium,
	Short,
}

//Type aliasing
#[allow(dead_code)]
type Adventure = Quest;

//Rust struct
#[derive(Debug)]
pub struct Secrets {
	pub x: i32,
	pub y: i32,
}

pub fn time() {
	println!("{}", chrono::Local::now());
}

pub fn sayGoodbye() {
	println!("Goodbye");
}

pub fn printmul() {
	let x = 42;
	let y = -69;
	let xy = x * y;
	println!("{} * {} = {}", x, y, xy)
}