struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![i32::MAX],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.min_stack.push(*self.min_stack.last().unwrap().min(&val));
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

