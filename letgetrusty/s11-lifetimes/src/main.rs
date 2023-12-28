fn main() {
    let mut string1 = String::from("abcd");
    let mut string2 = String::from("xyz");

    let result = longest(&string1.as_str(), string2.as_str());
    // let result = longest(&mut string1, &mut string2);
    println!("the longest string is {}", result);
    println!("string1 after {}",string1);

}

fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}