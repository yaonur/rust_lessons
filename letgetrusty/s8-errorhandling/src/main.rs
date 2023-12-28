fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(x:i32){
    if x ==22{
        panic!("Don't use 22!");
    }
}