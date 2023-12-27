
pub mod serving {
    use crate::hosting::add_to_waitlist;

    fn take_order() {
		println!("takeorder then serve");
		add_to_waitlist();
	}
    fn serve_order() {}
    fn take_payment() {}
}
