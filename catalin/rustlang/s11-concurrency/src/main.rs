use std::{
    thread::{self, sleep},
    time::Duration,
    sync::mpsc,
    sync::{Mutex,Arc},
};

const NUM_THREDS: usize =20;

fn start_thread(d:usize,tx: mpsc::Sender<usize>){
    thread::spawn(move || {
        println!("setting timer {}!",d);
        thread::sleep( Duration::from_secs(d as u64));
        println!("sending timer {}!",d);
        tx.send(d).unwrap();
    });
}
fn main() {
    // let mut threads = vec![];
    // for i in 0..10 {
    //     let th = thread::spawn(move || {
    //         // println!("new thread");
    //         sleep(Duration::from_millis(i * 1000));
    //         println!("Hello from thread number {}", i)
    //     });
    //     threads.push(th)
    // }

    // for t in threads {
    //     t.join();
    // }

    // println!("Main thread");

    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    //     tx.send(42).unwrap();
    // });
    // println!("received {}",rx.recv().unwrap());

    // let (tx,rx) = mpsc::channel();
    // for i in 0..NUM_THREDS {
    //     start_thread(i,tx.clone());
    // }
    // for j in rx.iter().take(NUM_THREDS){
    //     println!("received {}",j);
    // }

    let c = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for i in 0..10{
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
        });
        threads.push(t)
    }

    for th in threads {
        th.join().unwrap();
    }
    println!("Result: {}",*c.lock().unwrap());



}
