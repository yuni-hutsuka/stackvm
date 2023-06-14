use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Welcome to My Stack-VM.");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("In the {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file contents");

    println!("With text:\n{}", contents);
}
