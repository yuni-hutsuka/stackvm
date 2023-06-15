use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Stack {
    stack: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn push(&mut self, value: i32) {
        self.stack.push(value)
    }
    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn length(&mut self) -> usize {
        self.stack.len()
    }
}

fn main() {
    println!("Welcome to My Stack-VM.");

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("target name is: {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file contents");

    println!("With text:\n```\n{}\n```", contents);

    let mut stack: Stack = Stack::new();

    stack.push(10);
    println!("I pushed 10");
    let tmp: i32 = stack.pop().unwrap();
    println!("I poped and result is {}", tmp);
    println!("I think stack is empty now. -> {:?}", stack.is_empty());
    stack.push(10);
    println!("I pushed 10");
    stack.push(-10);
    println!("I pushed -10");
    println!("I think stack length is 2 now. -> {:?}", stack.length());
    let tmp: i32 = stack.pop().unwrap();
    println!("I poped and result is {}", tmp);
    let tmp: i32 = stack.pop().unwrap();
    println!("I poped and result is {}", tmp);
    println!("I think stack is empty now. -> {:?}", stack.is_empty());
    println!("I think next action is caused panic! -> {}", stack.pop().unwrap());
}
