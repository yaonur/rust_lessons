#[allow(unused_variables)]
#[allow(unused_assignments)]
use std::ops::Add;
use std::vec;
mod generics;
mod returning_traits;
mod traits;
mod static_dispatch;
use generics::{bark_it, Dog};
use returning_traits::{get_animal, Animal, Cat as TrCat, Dog as TrDog};
use traits::{Developer, JavaDev, RustDev};
use static_dispatch::{Duplicateable,duplicate};

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output =Point;

    fn add(self, other :Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    // let r = RustDev {awesome: true};
    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language());
    r.say_hello();
    println!("{}", j.language());
    j.say_hello();

    // generics ------------------
    let dog = Dog {
        species: "retiriever",
    };
    // let cat = Cat {color: "black"};

    bark_it(dog);
    // bark_it(cat); // wont work

    // returning traits -------------
    println!("The animal says {}", get_animal(0.2).make_noise());
    println!("The animal says {}", get_animal(1.2).make_noise());

    // adding traits to existing types ----------------
    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
    // let b = vec!["hello", "world"];
    // println!("sum = {}", b.sum()); // wont work because of type mismatch

    // operator overloading ----------------
    let p1 = Point {x: 1.0, y: 2.0};
    let p2 = Point {x: 3.0, y: 4.0};
    // let p3=p1.add(p2);
    let p3=p1 + p2;
    println!("added Pointer is:{:?}", p3);


    // static dispatch ----------------

    let a=42;
    let b ="Hey John".to_string();
    duplicate(a);
    duplicate(b);
}
