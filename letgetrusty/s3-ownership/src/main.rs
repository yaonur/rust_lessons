#[allow(unused_variables)]
fn main() {
    // Each value in Rust has a variable that's called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
    {
        // let s = "hello";
        let s = String::from("hello");
    }// this scope is over

    //  primitive types are stored on the stack 
    // and will be copied when assigned to another variable by default
    let x  =5;
    let y =x; // Copy

    let move1 = String::from("hello");
    let move2 = move1; // Move

    // println!("{}, world!", move1); // this wont work

    let copy1 = String::from("copying");
    let copy2 = copy1.clone(); // Clone 
    println!("copy1 = {}, copy2 = {}", copy1, copy2);


    // Ownership and Functions
    let owner1 = String::from("Owner");
    takes_ownership(owner1);
    // println!("owner1 = {}", owner1); // this wont work because it was moved

    let x = 5;
    makes_copy(x);
    println!("x = {}", x); // this will work because it was copied

    // Return Values and Scope
    let s1 = gives_ownership();
    println!("s1 owner is moved here = {}", s1);

    let s2 = String::from("will be back");   
    let s3 = takes_and_gives_back(s2);
    println!("s2 owner is moved here to s3 = {}", s3);
    // println!("s2 is not valid anymore {}",s2); // this wont work because it was moved

    // referencing borrowing
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!("we can use s1 again couse we passed it as reference: {}", s1);
    change_reference(&mut s1);
    println!("changed s1 on the function: {}", s1);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;// cant have two mutable references in the same scope
    // println!("r1 = {}, r2 = {}", r1, r2); // this wont work 

    // dangling references
    // let reference_to_nothing = dangle(); // wont work because s is dropped

    // Slices ---------------
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("the word is: {}",s);
    println!("first word in {} is: {}", s,word);

    let a  = [1,2,3,4,5];
    let slice = &a[1..3];
    

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("function take this string:{}", a_string);
    println!("will give back it now");
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_reference(s : &mut String) {
    s.clear();
    s.push_str("new value of s")
}

fn first_word (s:&String) -> &str {
    let bytes = s.as_bytes();

    for (i,&item ) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, and is dropped. Its memory goes away.

