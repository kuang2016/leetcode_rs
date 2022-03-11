pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.output.is_empty() {
            while !self.input.is_empty() {
                self.output.push(self.input.pop().unwrap());
            }
        }
        return self.output.pop().unwrap();
    }

    fn peek(&self) -> i32 {
        if self.output.is_empty() {
            self.input.first().cloned().unwrap()
        } else {
            self.output.last().cloned().unwrap()
        }
    }

    fn empty(&self) -> bool {
        return self.output.is_empty() && self.input.is_empty();
    }
}
