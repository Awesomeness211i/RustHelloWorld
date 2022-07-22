
mod example {

	pub fn fibbonachi(n:u32) -> u32 {
		let mut fprev = 0;
		let mut fcurr = 1;
		if n == 0 {
			return fprev;
		}
		else if n == 1 {
			return fcurr;
		}
		for i in 1..n {
			let fnext = fprev + fcurr;
			fprev = fcurr;
			fcurr = fnext;
		}
		return fcurr;
		//recursion
		return fibbonachi(n-1) +  fibbonachi(n - 2);
	}

	#[cfg(test)]
	mod tests {
		//Test driven development
		#[test]
		fn test() {
			//assert!(false);
			//assert_eq!(1, 0);
			//assert_ne!(0, 0);
		}
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

	//Made Test.rs with test function in example directory
	pub mod test;
	//Testing aliasing
	pub use test::test2;
	//renaming something using aliasing
	pub use test::test3 as three;

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
}

fn main() {
	let num = 25;
	println!("Hello, world! {}", num);
	example::sayGoodbye();
	example::time();
	example::test::test();
	example::test2();
	example::three();
	example::printmul();
	let st = example::Secrets { x: 42, y: 64 };
	println!("{:?}", st);
	let _q = example::Quest::Long;
	let _nothing = ();
	println!("Answer: {}", example::answer(1, 2));
	println!("42nd fibbonachi number: {}", example::fibbonachi(42));

	//example for loop
	for x in 0..10 {
		println!("{}", x);
	}
}