use std::env;

fn main() {
    println!("Welcome to My Stack-VM.");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
