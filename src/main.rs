mod stack;
mod input;

use std::env;

fn main() {
    // 引数受け取り
    let args: Vec<String> = env::args().collect();

    // ファイルオープン
    let contents: String = input::input::read_file(&args[1]);
    // contentsを分割
    let lines = contents.lines().collect::<Vec<&str>>();
    // initial_load
    let (program_counter, label_counter) = input::input::initial_load(lines);

    let mut pointer: usize = 0;
    let mut stack = stack::stack::Stack::new();

    while pointer < program_counter.len() {
        let tmp = &program_counter[pointer];
        let cmd = tmp[0].to_string();

        match &*cmd {
            "push" => {
                println!("push! -> {}", tmp[1]);
                stack.push(tmp[1].parse::<i32>().unwrap());
            },
            "pop" => {
                if stack.is_empty(){
                    println!("Stack is empty!");
                    pointer += 1;
                    continue;
                }
                let result = stack.pop();
                println!("popped! -> {}", result.unwrap());
            },
            "add" => {
                if stack.length() < 2 {
                    println!("lower!");
                    pointer += 1;
                    continue;
                }
                let result = stack.pop().unwrap() + stack.pop().unwrap();
                println!("added! -> {}", result);
                stack.push(result);
            },
            "sub" => {
                if stack.length() < 2 {
                    println!("lower!");
                    pointer += 1;
                    continue;
                }
                let result = stack.pop().unwrap() - stack.pop().unwrap();
                println!("subtracted! -> {}", result);
                stack.push(result);
            },
            "mul" => {
                if stack.length() < 2 {
                    println!("lower!");
                    pointer += 1;
                    continue;
                }
                let result = stack.pop().unwrap() * stack.pop().unwrap();
                println!("multipled! -> {}", result);
                stack.push(result);
            },
            "div" => {
                if stack.length() < 2 {
                    println!("lower!");
                    pointer += 1;
                    continue;
                }
                let result = stack.pop().unwrap() / stack.pop().unwrap();
                println!("divided! -> {}", result);
                stack.push(result);
            },
            "equal" => {
                println!("compare args and stack-top!");
                let stack_top = stack.pop().unwrap();
                stack.push(stack_top);
                if tmp[1].parse::<i32>().unwrap() == stack_top {
                    println!("there are equal!");
                    stack.push(1);
                } else {
                    println!("there are not equal!");
                    stack.push(0);
                }
            },
            "lt" => {
                println!("compare args and stack-top!");
                let stack_top = stack.pop().unwrap();
                stack.push(stack_top);
                if tmp[1].parse::<i32>().unwrap() > stack_top {
                    println!("stack-top is less than arg {}!", tmp[1].parse::<i32>().unwrap());
                    stack.push(1);
                } else {
                    println!("stack-top is not less than arg {}!", tmp[1].parse::<i32>().unwrap());
                    stack.push(0);
                }
            },
            "gt" => {
                println!("compare args and stack-top!");
                let stack_top = stack.pop().unwrap();
                stack.push(stack_top);
                if tmp[1].parse::<i32>().unwrap() < stack_top {
                    println!("stack-top is greater than arg {}!", tmp[1].parse::<i32>().unwrap());
                    stack.push(1);
                } else {
                    println!("stack-top is not greater than arg {}!", tmp[1].parse::<i32>().unwrap());
                    stack.push(0);
                }
            },
            "jump" => {
                println!("hop, step, jump! to {}", tmp[1].to_owned());
                let value: usize = *label_counter.get(tmp[1]).expect("unknow label!") as usize;
                pointer = value;
            },
            "jump_if" => {
                println!("if 0, jump to labe! if 1, through!");
                let bool_num = stack.pop().unwrap();
                if bool_num == 0 {
                    println!("0, 0, 0! hop, step, jumping!");
                    let value: usize = *label_counter.get(tmp[1]).expect("unknow label!") as usize;
                    pointer = value;
                } else {
                    println!("1, 1, 1! through!");
                }
            },
            "label" => {
                println!("label!");
            },
            _ => {
                println!("unknown cmd...");
            },
        }

        pointer += 1;
    }
}
