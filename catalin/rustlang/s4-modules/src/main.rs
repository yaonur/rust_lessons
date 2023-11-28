mod player;

fn main() {
    println!("Hello, world!");
    player::play_movie("The Matrix");
    player::play_audio("The Matrix");
    clean::clean_house();
    clean::files::clean_files();
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