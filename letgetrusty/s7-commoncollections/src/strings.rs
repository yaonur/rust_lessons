use unicode_segmentation::UnicodeSegmentation;
#[allow(unused_variables)]
pub fn run_strings() {
	// Strings are stored as a collection of UTF-8 encoded bytes
	let s1 = String::new();
	let s2 ="initial contents";
	let s3 = s2.to_string();
	let s4 = String::from("initial contents");

	let mut s = String::from("foo");
	s.push_str("bar");
	s.push('!');
	println!("{}", s);
	let s1 = String::from("Hello, ");
	let s2= String::from("world!");
	let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
	// println!("{}", s1); // s1 is move so it cant be used 
	println!("s2: {}", s2);
	println!("{}", s3);
	let s1 = String::from("with format");
	let s2 = String::from("macro its doesnt take owner ship");
	let s3 = format!("{} {}", s1, s2);
	println!("this is s3:{}", s3);
	println!("we can still use s1:{}", s1);
	let hello = String::from("Здравствуйте");
	for b in hello.bytes() {
		println!("{}", b);
	}
	for g in "नमस्ते".graphemes(true) {
		println!("{}", g);
	}
}