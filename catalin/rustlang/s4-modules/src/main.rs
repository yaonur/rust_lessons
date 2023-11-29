use crate::archive::arch::arch_file as arc;
use rand::Rng;
mod player;
mod archive;

fn main() {
    println!("Hello, world!");
    player::play_movie("The Matrix");
    player::play_audio("The Matrix");
    clean::clean_house();
    clean::files::clean_files();
    arc("The Matrix");
    let mut rng=rand::thread_rng();
    let a:i32=rng.gen();
    println!("a={}",a);
}

mod clean {
    pub fn clean_house() {
        println!("clean house");
    }
    pub mod files {
        pub fn clean_files() {
            println!("clean files");
        }
    }
}