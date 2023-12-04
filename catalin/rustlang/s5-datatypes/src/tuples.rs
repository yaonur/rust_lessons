pub fn tuples_run() {
    // static size
    // element values can be updated
    // indexed
    // limited to 12 elements
    let mut person: (&str, i64, bool) = ("John", 27, true);
    println!("person:{:?}", person);
    person.0 = "jack";
    println!("person.0:{:?}", person.0);
	// destructuring
    let (name, age, employed) = person;
	println!("name:{}, age:{}, employed:{}", name, age, employed);
}
