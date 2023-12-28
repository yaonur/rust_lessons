mod structs;
use std::vec;
use structs::run_structs;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest_num = number_list[0];

    find_largest_num(number_list, &mut largest_num);

    println!("The largest_num number is {}", largest_num);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let mut largest_char = char_list[0];
    find_largest_char(char_list, &mut largest_char);
    println!("The largest_char char is {}", largest_char);

    let largest = find_largest_generic(vec![34, 50, 25, 100, 65]);
    println!("The largest is {}", largest);
    let largest  = find_largest_generic( vec!['x', 'm', 'a', 'q']);
    println!("The largest is {}", largest);

    run_structs();
}

fn find_largest_num(number_list: Vec<i32>, largest: &mut i32) {
    for number in number_list {
        if number > *largest {
            *largest = number;
        }
    }
}

fn find_largest_char(char_list: Vec<char>, largest: &mut char) {
    for char in char_list {
        if char > *largest {
            *largest = char;
        }
    }
}

fn find_largest_generic<T: PartialOrd + Copy>(some_list: Vec<T>) -> T {
    let mut largest = some_list[0];
    for l_element in some_list {
        if l_element > largest {
            largest = l_element;
        }
    }
    largest
}
