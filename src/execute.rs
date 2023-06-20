mod stack;
mod stack_control;
mod binomial_calculus;

pub(crate) mod execution {
    use std::collections::HashMap;
    use super::{stack::{self, Stack}, stack_control};

    pub(crate) fn execution(program_counter: Vec<Vec<&str>>, label_counter: HashMap<String, i32>) {
        let mut pointer: usize = 0;
        let mut stack: Stack<String> = stack::Stack::new();

        while pointer < program_counter.len() {
            let tmp = &program_counter[pointer];
            let types: String = tmp[1].to_string();
            let value: String = tmp[2].to_string();
            let cmd = tmp[0].to_string();

            match &*cmd {
                "push" => stack_control::push(stack, value),
                "pop" => {
                },
                "add" => {
                },
                "sub" => {
                },
                "mul" => {
                },
                "div" => {
                },
                "equal" => {
                },
                "lt" => {
                },
                "gt" => {
                },
                "jump" => {
                },
                "jump_if" => {
                },
                "label" => {
                },
                _ => {
                    println!("unknown cmd...");
                },
            }
            pointer += 1;
        }
    }
}
