pub struct Solution;

use std::collections::VecDeque;

struct MyStack {
    stack1: VecDeque<i32>,
    _stack2: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyStack {
    fn new() -> Self {
        Self {
            stack1: VecDeque::new(),
            _stack2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let n = self.stack1.len();
        self.stack1.push_back(x);
        for _ in 0..n {
            let v = self.stack1.pop_front().unwrap();
            self.stack1.push_back(v);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack1.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.stack1.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert_eq!(obj.pop(), 2);
        assert!(!obj.empty());
    }
}
