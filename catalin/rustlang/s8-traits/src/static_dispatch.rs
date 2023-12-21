
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

pub fn duplicate<T: Duplicateable> (x:T){
	println!("{}",x.dupl());
}