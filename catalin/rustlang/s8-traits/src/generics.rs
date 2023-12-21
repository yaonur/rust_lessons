pub trait Bark {
	fn bark(&self) -> String;
}

pub struct Dog {
	pub species: &'static str
}

pub struct Cat {
	pub color: &'static str
}

impl Bark for Dog {
	fn bark(&self) -> String {
		format!("{} says: Woof!", self.species)
	}
}

pub fn bark_it<T: Bark>(barker: T) {
	println!("{}", barker.bark());
}