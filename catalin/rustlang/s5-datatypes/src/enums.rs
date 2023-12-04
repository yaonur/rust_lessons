#[allow(dead_code)]
#[allow(unused_variables)]
pub fn enums_run() {
    print!("Color:{}\n", Colors::Red as i32);
    let my_color = Colors::Blue;
    println!("my_color:{:?}", my_color);
	let person = Person::Name(String::from("John"));
	println!("person:{:?}", person);
	let person= Person::Surname(String::from("Doe"));
	println!("person:{:?}", person);
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(i32),
}
