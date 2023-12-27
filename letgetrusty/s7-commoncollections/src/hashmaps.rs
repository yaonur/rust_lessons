use std::collections::HashMap;
pub fn run_hashmaps(){

	let blue = String::from("blue");
	let yellow = String::from("yellow");

	let mut scores = HashMap::new();

	scores.insert(blue, 10);
	scores.insert(yellow, 50);

	let team_name = String::from("blue");
	let score = scores.get(&team_name);
	println!("scores is:{:?}", scores);
	println!("score is:{:?}", score);
	scores.entry(String::from("green")).or_insert(30);
	scores.entry(String::from("green")).or_insert(50);
	println!("scores after using entry is:{:?}", scores);

	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("map is:{:?}", map);

}