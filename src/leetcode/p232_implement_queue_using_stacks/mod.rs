pub struct Solution;

struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack2.is_empty() {
            while let Some(v) = self.stack1.pop() {
                self.stack2.push(v);
            }
        }
        self.stack2.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.stack2.is_empty() {
            while let Some(v) = self.stack1.pop() {
                self.stack2.push(v);
            }
        }
        *self.stack2.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert!(!obj.empty());
    }
}
