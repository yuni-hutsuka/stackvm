pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn push(&mut self, types: String, value: String) {
        if types == "int" {
            self.stack.push(value.to_string())
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn length(&mut self) -> usize {
        self.stack.len()
    }
}
