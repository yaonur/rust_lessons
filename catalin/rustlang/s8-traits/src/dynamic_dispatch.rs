pub trait Duplicateable {
	fn dupl(&self) -> String;
}

impl Duplicateable for String {
	fn dupl(&self) -> String {
		format!("{0}{0}",*self)
	}
}

impl Duplicateable for i32 {
	fn dupl(&self) -> String {
		format!("{}",*self * 2)
	}
}

// pub fn duplicate<T: Duplicateable> (x:T){
// 	println!("{}",x.dupl());
// }

// this will convert at run time
fn duplicate(x:&dyn Duplicateable) {
	println!("{}",x.dupl());
}

fn main() {
	let s = String::from("Hello");
	let i = 5;
	duplicate(&s);
	duplicate(&i);
}