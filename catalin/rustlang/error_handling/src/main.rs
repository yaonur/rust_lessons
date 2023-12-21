use std::io;
#[allow(unused_imports)]
use std::{
    fs::{remove_file, File, OpenOptions},
    io::{Read, Write},
};
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => return Err(e),
//     }
// }

fn read_username_from_file()-> Result<String,io::Error>{
    let mut f = File::open("username.txt")?;
    let mut s = String::new(); 
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    // let mut file = File::create("hey.txt").expect("Could not create file");
    // file.write_all(b"Hello, world!").expect("Could not write file");
    // let mut file = OpenOptions::new().append(true).open("hey.txt").expect("Could not open file");
    // file.write_all("Adding content to the file.\n".as_bytes()).expect("Could not write file");

    // let mut file = File::open("hey.txt").expect("Could not open file");
    // let mut contents = String::new();
    // file.read_to_string( &mut contents).expect("Could not read file");
    // println!("{}",contents);

    // remove_file("hey.txt").expect("Could not remove file");
    // let f = File::open("main.jpg");
    // match f {
    //     Ok(f) => {
    //         println!("File opened successfully.");
    //     },
    //     Err(e) => {
    //         println!("Error opening file.");
    //     }
    // }

    // divide(Some(1));
    // divide(Some(10));
    // divide(None);
    // divide(Some(0));
    let a = read_username_from_file();
    println!("{:?}", a);
}
// const ANSWER_TO_LIFE: u8 = 42;
// fn divide(x: Option<i32>) {
//     match x {
//         Some(0) => panic!("Can't divide by zero!"),
//         Some(x) => println!("result is {}", (ANSWER_TO_LIFE as i32) / x),
//         None => println!("None received, the answer is {}", ANSWER_TO_LIFE),
//     }
// }
