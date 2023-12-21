#[derive(Debug)]
struct Person {
	name: String
}

#[derive(Debug)]
struct Dog<'l> {
	name : String,
	owner: &'l Person
}

impl Person {
	// fn get_name(&self) -> &String {
		fn get_name<'l>(&'l self)->&'l String{
		&self.name
	}
}

fn main() {
	let p1 = Person { name: String::from("John") };
	let d1 = Dog { name: String::from("Rusty"), owner: &p1 };
	 println!("{:?}", d1);

	 let mut a :&String;
	 {
		let p2 = Person {name:String::from("Jane")};
		// a = p2.get_name();// this will not work because p2 is dropped at the end of this block
		a = p1.get_name();
	 }
	 println!("{}",a);
}

fn get_str() -> &'static str {
	"Hello"
}