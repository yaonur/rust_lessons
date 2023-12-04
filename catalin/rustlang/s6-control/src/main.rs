use crate::Suit::{Clubs, Diamonds, Hearts, Spades};
use rand::Rng;
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..10);
    if num > 5 {
        println!("{} is greater than 5", num);
    } else if num == 5 {
        println!("{} is equal to 5", num);
    } else {
        println!("{} is less than 5", num);
    }
    let res = if num >= 5 { true } else { false };
    println!("{} is greater or equal to 5 {}", num, res);

    print_choice(Hearts);
    print_choice(Diamonds);
    print_choice(Spades);
    print_choice(Clubs);

    country(44);

    for i in 0..10 {
        println!("{}.  {} ", i, get_oranges(i));
    }

    // let point =(0,0);
    // let point =(0,6);
    // let point =(6,0);
    let point =(6,6);
    match point {
        (0,0) => println!("origin"),
        (x,0) => println!("y axis, x = {}",x),
        (0,y) => println!("x axis, y = {}",y),
        (x,y) => println!("x = {}, y = {}",x,y),
    }

    // for loops
    for i in 1..11{
        println!("i = {}",i);
    }
    let pets =["cat","dog","bear","hamster","rabbit"];
    for pet in pets.iter(){
        if pet ==&"dog" {
            println!("this pet barks too much pet is:{}",pet);
            continue
        }
        if pet == &"bear"{
            println!("this not a pet its a bear breaking loop: :{}",pet);
            break
        }
        println!("I have a {}",pet);
    }
    for (pos,i) in (30..41).enumerate(){
        println!("{}:{}",pos,i);
    }

    
    // while loops
    get_squares(120);
    get_cubes(400);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "Unknown",
        _ => "Invalid",
    };
    println!("The country with code {} is {}", code, country);
}
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

fn print_choice(choice: Suit) {
    match choice {
        Suit::Clubs => println!("\u{2663}"),
        Suit::Diamonds => println!("\u{2666}"),
        Suit::Hearts => println!("\u{2665}"),
        Suit::Spades => println!("\u{2660}"),
    }
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "You have no oranges",
        1 | 2 => "You have one or two oranges",
        1..=5 => "You have some oranges",
        _ if (amount % 2 == 0) => "You have an even number of oranges",
        _ => "You have a lot of oranges",
    };
}

fn get_squares(limit:i32){
    let mut x =1;
    while x*x < limit{
        println!("{} square is {}",x,x*x);
        x+=1;
    }
}

fn get_cubes(limit:i32){
    let mut x =1;
    loop {
        println!("{} cube is {}",x,x*x*x);
        x+=1;
        if x*x*x > limit{
            break;
        }
    }
}
