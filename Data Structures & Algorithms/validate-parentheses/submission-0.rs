impl Solution {
    pub fn is_valid(s: String) -> bool {
        // if s = '[(({}))]
        // if 
        // we can have a stack and put in it any opening bracket and on the first closing branch we see, we compare it with the top of the stack is it 
        // the matching bracket for it, if yes pop out that opening one and traverse to continue, if stack at the end is empty then return true
        // if not return false
        // if the string started with closing return false right away
        // we can use Vec<char> yes as there is no built in stack in Rust, using the push and pop methods
        
        // if the string length is odd return false, this means imbalance
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => if stack.pop() != Some('(') { return false; },
                ']' => if stack.pop() != Some('[') { return false; },
                '}' => if stack.pop() != Some('{') { return false; },
                _ => {}
            }
        }

        stack.is_empty()
    }
}
