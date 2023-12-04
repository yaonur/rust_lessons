pub fn tuples_run() {
	// static size
	// element values can be updated
	// indexed
	// limited to 12 elements
    let person:(&str, i64, bool) = ("John", 27, true);
    println!("person:{:?}", person);
	println!("person.0:{:?}", person.0);
}
