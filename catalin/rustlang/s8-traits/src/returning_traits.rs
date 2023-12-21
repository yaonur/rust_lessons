

pub struct Dog {}
pub struct Cat {}
pub trait Animal {
	fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
	fn make_noise(&self) -> &'static str {
		"Woof!"
	}
}

impl Animal for Cat {
	fn make_noise(&self) -> &'static str {
		"Meow!"
	}
}

pub fn get_animal(rang_number:f64) -> Box<dyn Animal> {
	if rang_number  < 1.0 {
		Box::new(Dog {})
	} else {
		Box::new(Cat {})
	}
}