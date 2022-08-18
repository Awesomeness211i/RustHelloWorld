
pub enum Option<T> {
	Some(T),
	None,
}

pub enum Result<T, E> {
	Ok(T),
	Err(E),
}

pub struct Point<T, U> {
	pub x: T,
	pub y: U,
}

impl<T, U> Point<T, U> {
	pub fn x(&self) -> &T {
		return &self.x;
	}
	pub fn y(&self) -> &U {
		return &self.y;
	}
	pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
		return Point{x: self.x, y: other.y};
	}
}

impl<T: std::fmt::Display, U: std::fmt::Display> Point<T, U> {
	pub fn print(&self) {
		println!("{}, {}", self.x, self.y);
	}
}

pub fn get_largest<T: PartialOrd + Copy>(vector: Vec<T>) -> T {
	let mut largest = vector[0];
	for item in vector {
		if item > largest {
			largest = item;
		}
	}
	return largest;
}