#![allow(non_snake_case)]

#[cfg(test)]
pub mod unittest;

pub mod example;
pub mod generics;
pub mod input;
pub mod network;

//using the standard library
use std::{thread, sync::{mpsc}};

//testing dependencies
use rand::Rng;
use colored::*;

//Made test.rs for mod test
pub mod test;
//Testing aliasing
pub use test::test2;
//renaming something using aliasing
pub use test::test3 as three;

trait ApplicationLayer {
	fn OnAdd(self: &Self);
	fn OnRemove(self: &Self);
	fn OnEvent(self: &Self);

	fn Update(self: &Self);
	fn Render(self: &Self);

	fn GetUUID(&self) -> u64;
}

struct LayerStack {
	layers: Vec<Box<dyn ApplicationLayer>>,
	layerInsertIndex: usize,
	overlayInsertIndex: usize,
}

impl LayerStack {
	pub fn new() -> Self {
		return Self {
			layers: Vec::new(),
			layerInsertIndex: 0,
			overlayInsertIndex: 0,
		};
	}
	pub fn iter(&self) -> impl Iterator<Item = &Box<dyn ApplicationLayer>> {
		return self.layers.iter().rev();
	}

	pub fn AddOverlay(&mut self, layer: Box<dyn ApplicationLayer>) {
		layer.OnAdd();
		self.layers.push(layer);
		self.overlayInsertIndex += 1;
	}
	pub fn RemoveOverlay(&mut self, uuid: u64) {
		if let Some(i) = self.layers[self.layerInsertIndex..self.overlayInsertIndex].iter().position(|l| { return l.GetUUID() == uuid; }) {
			self.layers[i].OnRemove();
			self.layers.remove(i);
			self.overlayInsertIndex -= 1;
		}
	}
	pub fn AddLayer(self: &mut Self, layer: Box<dyn ApplicationLayer>) {
		layer.OnAdd();
		self.layers.insert(self.layerInsertIndex, layer);
		self.layerInsertIndex += 1;
	}
	pub fn RemoveLayer(self: &mut Self, uuid: u64) {
		if let Some(i) = self.layers[..self.layerInsertIndex].iter().position(|l| { return l.GetUUID() == uuid; }) {
			self.layers[i].OnRemove();
			self.layers.remove(i);
			self.layerInsertIndex -= 1;
		}
	}
}

struct ExampleLayer {
	uuid: u64,
}

impl ExampleLayer {
	pub fn new() -> ExampleLayer {
		return ExampleLayer {
			uuid: rand::thread_rng().gen(),
		};
	}
}

impl ApplicationLayer for ExampleLayer {
	fn OnAdd(self: &Self) {
		println!("OnAdd: {}", self.uuid);
	}
	fn OnRemove(self: &Self) {
		println!("OnRemove: {}", self.uuid);
	}
	fn OnEvent(self: &Self) {
		println!("OnEvent: {}", self.uuid);
	}
	fn Update(self: &Self) {
		println!("Update: {}", self.uuid);
	}
	fn Render(self: &Self) {
		println!("Render: {}", self.uuid);
	}
	fn GetUUID(&self) -> u64 {
		return self.uuid;
	}
}

fn main() {
	//testing layerstack things
	let mut layerstack = LayerStack::new();
	let example0 = Box::new(ExampleLayer::new());
	let example1 = Box::new(ExampleLayer::new());
	let example2 = Box::new(ExampleLayer::new());
	let uuid0 = example0.GetUUID();
	let uuid1 = example1.GetUUID();
	let uuid2 = example2.GetUUID();
	layerstack.AddLayer(example0);
	layerstack.AddOverlay(example1);
	layerstack.AddLayer(example2);
	for layer in layerstack.iter() {
		layer.Update();
		layer.Render();
		layer.OnEvent();
	}
	layerstack.RemoveLayer(uuid0);
	layerstack.RemoveLayer(uuid2);
	layerstack.RemoveOverlay(uuid1);

	//testing concurrency
	let (transmitter, reciever) = mpsc::channel();
	let transmitter2 = transmitter.clone();

	let handle = thread::spawn(move || {
		let val = String::from("Hello World");
		transmitter.send(val).unwrap();
	});

	let recieved = reciever.recv().unwrap();
	println!("Got: {recieved}");

	println!("Other Tests:");
	//testing print formatting
	let num = 25;
	println!("Hello, world! {}", num);
	//testing example and test modules
	example::sayGoodbye();
	example::printmul();
	example::time();
	test::test();
	test::test2();
	test::test3();
	//testing aliasing
	test2();
	three();
	//testing types
	let st = example::Secrets { x: 42, y: 64 };
	println!("{:?}", st);
	let _q = example::Quest::Long;
	let _nothing = ();
	//testing functions
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
	//testing generics
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

	/*testing getting input
	let mut in1 = String::new();
	input::getconsoleinput(&mut in1);
	println!("Your input: {}", in1);*/
	//random numbers
	let random_number = rand::thread_rng().gen_range(-1.0..1.0);
	println!("Random Number: {}", random_number);
	//testing colored output
	let color = Color::TrueColor{ r: 255, g: 0, b: 255 };
	println!("{}","Hello".color(color));
	let col = "NOPE".to_owned().color(color);
	println!("{}", col);
	//network
	let a = network::Ipa::v4(0, 0, 0, 0);
	println!("{}", a.to_string());
	let b = network::Ipa::v6(0, 0, 0, 0, 0, 0, 0, 0);
	println!("{}", b.to_string());
}