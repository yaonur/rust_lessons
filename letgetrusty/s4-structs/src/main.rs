struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // to print struct
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self , other: &Reactangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Reactangle {
    fn square(size: u32) -> Reactangle {
        Reactangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("somemail@mail.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let name = user1.username; // copy
    user1.username = String::from("newusername");
    println!("user1 name is: {}", name);

    let user2 = build_user(
        String::from("dumbaras@mail.com"),
        String::from("name_dumbaras"),
    );
    println!("user2 name is: {}", user2.username);

    // tuple structs

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Reactangle {
        width: 30,
        height: 50,
    };
    let rect_smaller = Reactangle {
        width: 10,
        height: 40,
    };
    let rect_bigger = Reactangle {
        width: 60,
        height: 100,
    };
    println!("rect is: {:#?}", rect);
    println!("rect area is: {}", rect.area());
    println!("rect can hold rect_smaller: {}", rect.can_hold(&rect_smaller));
    let square = Reactangle::square(10);
    println!("square is: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

