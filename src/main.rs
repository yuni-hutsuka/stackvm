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
    // 引数受け取り
    let args: Vec<String> = env::args().collect();

    // ファイルオープン
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file contents");

    // contentsを分割
    let splitted = contents.lines().collect::<Vec<&str>>();
    let mut program_counter = Vec::new();

    for line in splitted {
        let tmp = line.split_whitespace().collect::<Vec<&str>>();
        program_counter.push(tmp);
    }

    let mut stack: Stack = Stack::new();

    for i in 0..program_counter.len() {
        let tmp = &program_counter[i];
        let cmd = tmp[0].to_string();
        match &*cmd {
            "push" => {
                println!("push! -> {}", tmp[1]);
                stack.push(tmp[1].parse::<i32>().unwrap());
            },
            "pop" => {
                if stack.is_empty(){
                    println!("Stack is empty!");
                    continue;
                }
                let result = stack.pop();
                println!("popped! -> {}", result.unwrap());
            },
            "add" => {
                if stack.length() < 2 {
                    println!("lower!");
                    continue;
                }
                let result = stack.pop().unwrap() + stack.pop().unwrap();
                println!("added! -> {}", result);
                stack.push(result);
            },
            "sub" => {
                if stack.length() < 2 {
                    println!("lower!");
                    continue;
                }
                let result = stack.pop().unwrap() - stack.pop().unwrap();
                println!("subtracted! -> {}", result);
                stack.push(result);
            },
            "mul" => {
                if stack.length() < 2 {
                    println!("lower!");
                    continue;
                }
                let result = stack.pop().unwrap() * stack.pop().unwrap();
                println!("multipled! -> {}", result);
                stack.push(result);
            },
            "div" => {
                if stack.length() < 2 {
                    println!("lower!");
                    continue;
                }
                let result = stack.pop().unwrap() / stack.pop().unwrap();
                println!("divided! -> {}", result);
                stack.push(result);
            },
            _ => (),
        }
    }
}
