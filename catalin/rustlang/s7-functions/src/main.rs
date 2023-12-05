static mut R: i32 = 0;
fn main() {
    let mut name = "John";
    say_hi(&mut name);
    println!("new name: {}", name);
    {
        //scope
        let a = 3;
        println!("a: {}", a);
    }
    // println!("a: {}", a); //wont work
    unsafe {
        R = 5;
        println!("R: {}", R);
    }
    // closures
    let sum = |a: i32, b: i32| -> i32 { a + b };
    println!("sum: {}", sum(1, 2));
    // clojure can be generic
    let gen = |x| println!("generic: {}", x);
    gen(3);

    // gen(true); cant change type of generic

    let square = |a: i32| a * a;
    apply(square, 3);

    // calculate the sum of all the squares less than 500
    // only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("loop sum: {}", sum);
    // with HOFs
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum: {}", sum2);


    // macros
}

fn say_hi(name: &mut &str) {
    *name = "Jane";
    println!("Hi, {}!", name)
}

// horder functions
fn apply(f: fn(i32) -> i32, a: i32) {
    println!("apply: {}", f(a));
}
fn is_even(x: i32) -> bool {
    x % 2 == 0
}
