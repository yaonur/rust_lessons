#[allow(unused_assignments)]
#[allow(unused_variables)]
pub fn generics_run(){
	let p1: Point<i32> = Point {x: 10, y: 20};
	let p2: Point<f64> = Point {x: 1.25, y: 2.76};
	println!("p1:{:?}", p1);
	println!("p2:{:?}", p2);
	let c1 = Colors::Red("#f00");
	let c2 = Colors::Red(255);
	println!("c1:{:?}", c1);
	println!("c2:{:?}", c2);
	let p3 = Point2 {x: 10, y: 20.5};
	println!("p3:{:?}", p3);
}

#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}

#[derive(Debug)]
enum Colors<T> {
	Red(T),
	Green(T),
	Blue(T),
}

#[derive(Debug)]
struct Point2<T, U> {
	x: T,
	y: U,
}
