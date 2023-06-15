use std::env;
use std::fs::File;
use std::io::prelude::*;

// struct Stack {
//     stack: Vec<i32>,
// }

// impl Stack {
//     fn new() -> Self {
//         Stack { stack: Vec::new() }
//     }
//     fn push(&mut self, value: i32) {
//         self.stack.push(value)
//     }
//     fn pop(&mut self) -> Option<i32> {
//         self.stack.pop()
//     }
//     fn is_empty(&self) -> bool {
//         self.stack.is_empty()
//     }
//     fn length(&mut self) -> usize {
//         self.stack.len()
//     }
// }

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

    // let mut stack: Stack = Stack::new();
}
