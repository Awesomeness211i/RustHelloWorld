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
	//example of getting command-line args
	for arg in std::env::args() {
		println!("{}", arg);
	}

	let number_list = vec![1, 4, 2, 21, 53, 44];
	let largest = generics::get_largest(number_list);
	println!("{}", largest);

	let number_list = vec!['y', 'm', 'a', 'q'];
	let largest = generics::get_largest(number_list);
	println!("{}", largest);

	let p = generics::Point{x:1.0, y:2};
	p.print();
	let p1 = generics::Point{x:'.', y:'t'};
	p1.print();
	let p2 = p.mixup(p1);
	p2.print();
}