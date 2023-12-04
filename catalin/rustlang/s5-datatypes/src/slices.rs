pub fn slices_run(){
	// slice is a pointer to a block of memory
	// size is determined at runtime
	// can be used on arrays, vectors and strings
	let numbers: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
	let slice = &numbers[1..4];
	println!("slice:{:?}", slice);

	let mut colors =["red", "green", "blue", "yellow"];
	println!("colors:{:?}", colors);
	update_colors( &mut colors[2..4]);
	println!("colors after update:{:?}", colors);
}

fn update_colors(colors_slice: &mut [&str]){
	colors_slice[0] = "purple";
	colors_slice[1] = "pink";
}