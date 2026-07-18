struct MinStack {
    // need here to have two Vec one main and one for the getMin feature
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {

    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    // adds an element using the push method to push at the end of the vector, push it to the min_stack if it is <= the current min
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }
    }

    // remove the last element in the vector, remove it also from the min_stack
    pub fn pop(&mut self) {
        let popped = self.stack.pop().unwrap();

        if popped == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }
    
    // get the value of the last element using index
    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    // get the op value in the min_stack
    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}