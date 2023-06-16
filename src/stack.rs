pub(crate) mod stack {
    pub struct Stack {
        stack: Vec<i32>,
    }

    impl Stack {
        pub fn new() -> Self {
            Stack { stack: Vec::new() }
        }
        pub fn push(&mut self, value: i32) {
            self.stack.push(value)
        }
        pub fn pop(&mut self) -> Option<i32> {
            self.stack.pop()
        }
        pub fn is_empty(&self) -> bool {
            self.stack.is_empty()
        }
        pub fn length(&mut self) -> usize {
            self.stack.len()
        }
    }
}
