#[allow(dead_code)]
pub struct JavaDev {
    awesome: bool,
}

pub trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello from developer");
    }
}

#[allow(dead_code)]
pub struct RustDev {
    awesome: bool,
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }
    fn say_hello(&self) {
        println!("println!(\"Hello World!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        "Java"
    }
    fn say_hello(&self) {
        println!("System.out.println(\"Hello World!\");");
    }
}