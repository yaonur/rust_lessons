enum IpAddrKing {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKing,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn print_something() {
        println!("some function")
    }
}
enum SomeOption<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    let four = IpAddrKing::V4(127, 0, 0, 1);
    let six = IpAddrKing::V6(String::from("::1"));

    let localhost = IpAddr {
        kind: IpAddrKing::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1"),
    };
    let x = 5;
    let y = Some(5);
    // let sum  = x+y;// wont work y would have no value
    let sum = x + y.unwrap_or(0);

    let five = Some(5);
    let six = plus_one(five);
    let none  = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // None => None,
        _ => None,
    }
}