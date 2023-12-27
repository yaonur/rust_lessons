mod strings;
mod hashmaps;
use std::vec;
use strings::run_strings;

use hashmaps::run_hashmaps;

#[allow(unused_variables)]
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    {
        let v2 = vec![1, 2, 3, 4, 5];
    }
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ]; 
    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        _ => println!("Not an int")
    }
    run_strings();
    run_hashmaps();
}
