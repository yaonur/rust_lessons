mod slices;
mod tuples;

fn main() {
    //arrays
    let primes: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("primes[0]={}", primes[0]);
    println!("{:?}", doubles);

    let numbers = [0; 15];
    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    numbers[3] = 5;
    println!("{:?}", numbers);
    for number in numbers.iter() {
        println!("{}", number + 1);
    }
    println!("{:?}", numbers);

    //vectors

    let mut primes: Vec<i32> = vec![2, 3, 5, 7, 11, 13, 17, 19];
    println!("primes:{:?}", primes);
    primes.push(7);
    println!("primes after push:{:?}", primes);
    primes.remove(1);
    println!("primes after remove index 1:{:?}", primes);

    let mut numbers: Vec<i32> = vec![2; 10];
    println!("numbers:{:?}", numbers);
    const DEFAULT2: bool = true;
    let values: Vec<bool> = vec![DEFAULT2; 10];
    println!("values:{:?}", values);

    numbers[5] = 8;
    println!("numbers after updating index 5:{:?}", numbers);

    for number in numbers.iter() {
        println!("{}", number * number );
    }

    //slices
    slices::slices_run();

    //tuples
    tuples::tuples_run();
}
