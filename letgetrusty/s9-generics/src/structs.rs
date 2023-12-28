struct Point <T,U> {
	x: T,
	y: U,
}

struct Point2 <T> {
	x: T,
	y: T,
}
impl <U> Point2 <U> {
	fn x(&self) -> &U {
		&self.x
	}
}

impl Point2<f64> {
	fn y(&self) -> &f64 {
		&self.y
	}
}

impl <T,U> Point<T,U> {
	fn mixup<V,W>(self, other: Point<V,W>)-> Point<T,W>{
		Point {
			x: self.x,
			y: other.y,
		}
	}
}

#[allow(unused_variables)]
pub fn run_structs() {
	let p1 = Point { x: 1, y: 2 };
	println!("point is: {} {}", p1.x, p1.y);
	let p2 = Point { x: 1.0, y: 2.0 };
	let p3 = Point { x: 1, y: 4.4 };
	let p4 = Point2 { x: 1, y: 2 };
	p4.x();
	// p4.y();// its not available for i32
	let p5 = Point2 { x: 1.0, y: 2.0 };
	p5.x();
	p5.y();
	let pmixed=p1.mixup(p3);
	println!("point is: {} {}", pmixed.x, pmixed.y)
}