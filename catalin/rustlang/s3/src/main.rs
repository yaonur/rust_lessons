#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut name: &str = "One";
    let age: i32 = 42;
    let million: i32 = 1_000_000;
    println!("{}", million);
    let is_day = true;
    println!("{}", is_day);
    let char1: char = 'a';
    println!("{}", char1);
    let smiley_face: char = '\u{1F600}';
    println!("{}", smiley_face);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{},{},{}", tup.0, tup.1, tup.2);
    // Destructuring
    let (x, y, z) = tup;
    println!("{},{},{}", x, y, z);

    let cat: &str = "cat";
    let dog: &'static str = "dog";
    let amount: i128 = 2343784294738924327894;

    a_function(&mut name);
    println!("{}", name);

}

fn a_function(name: &mut &str){
    println!("{}", name);
    *name = "Two";
}