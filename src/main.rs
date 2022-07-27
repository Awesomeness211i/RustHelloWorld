#[cfg(test)]
pub mod unittest;

pub mod example;
pub mod generics;

//Made test.rs for mod test
pub mod test;
//Testing aliasing
pub use test::test2;
//renaming something using aliasing
pub use test::test3 as three;

fn main() {
	let num = 25;
	println!("Hello, world! {}", num);
	example::sayGoodbye();
	example::time();
	test::test();
	test::test2();
	three();
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
	for arg in std::env::args() {
		println!("{}", arg);
	}
}