mod life_times;
fn main() {
    let i = 5;
    let j = i;

    // i is still valid and usable here primitives are copied
    println!("{}", j);
    println!("{}", i);

    let v = vec![1, 2, 3];
    let w = v;
    println!("{:?}", w);
    // println!("{:?}",v); // v is moved to w and is no longer valid

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("{:?}", v);
        v
    };

    let w = foo(w);
    println!("{:?}", w);

    // borrowing----------------

    let mut a = 6;
    let b = &mut a;
    // println!("a first time:{}", a); // wont work because a is borrowed
    // a += 2; // wont work
    println!("b:{}", *b);
    *b += 2;
    println!("a:{}", a);
    // println!("b again:{}", b); // this will make the above line not work
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
        // v.push(6);// its already borrowed as immutable
    }

    //life times

}
