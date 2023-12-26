fn main() {
    let mut  x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const S_COUNT: u32 = 100_000;

    // shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y ="five";
    println!("The value of y is: {}", y);

    // tuples
    let tup = ("hello",10_000);
    let (greet, count) = tup;
    println!("{} {}", greet, count);
    let count2 = tup.1;
    println!(" {}", count2);

    let mut counter =0;
    loop {
        println!("again!: {}", counter);
        counter+=1;
        if counter == 10 {
            break;
        }
    }

    while counter != 0 {
        println!("while loop: {}", counter);
        counter-=1;
    }

    let a= [10,20,30,40,50];

    for element in a.iter(){
        println!("the value is: {}", element);
    }
    for number  in 1..4 {
        println!("the value is: {}", number);
    }
}
