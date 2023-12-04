pub fn structures_run() {
    let emp = Employee {
        name: String::from("John"),
        company: String::from("ABC"),
        age: 27,
    };

    println!("emp:{:?}", emp);
    println!("emp.name:{:?}", emp.name);
    println!("emp.fn_details():{:?}", emp.fn_details());
	println! ("Employee::static_fn_detail():{:?}", Employee::static_fn_detail());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn fn_details(&self) -> String {
        format!(
            "{} works at {} and is {} years old",
            &self.name, &self.company, &self.age
        )
    }

	fn static_fn_detail() -> String {
		String::from("static_fn_detail")
	}
}
