mod front_of_house;


// exporting for external use also
pub use front_of_house::hosting;
pub fn eat_at_restaurant(){
	// Absolute path
	hosting::add_to_waitlist();


}
