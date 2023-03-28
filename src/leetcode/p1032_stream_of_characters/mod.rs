use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
struct TrieNode {
    sons: Vec<Option<Rc<RefCell<TrieNode>>>>,
    is_end: bool,
    fail: Option<Rc<RefCell<TrieNode>>>,
}

impl Default for TrieNode {
    fn default() -> Self {
        TrieNode {
            sons: vec![None; 26],
            is_end: false,
            fail: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct StreamChecker {
    root: Rc<RefCell<TrieNode>>,
    now: Rc<RefCell<TrieNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut stream_checker = StreamChecker::default();
        for word in words {
            let mut p = stream_checker.root.clone();
            for byte in word.as_bytes().iter().map(|b| (*b - b'a') as usize) {
                if p.borrow().sons[byte].is_none() {
                    p.borrow_mut().sons[byte] = Some(Rc::new(RefCell::new(TrieNode::default())));
                }
                let t = p.borrow().sons[byte].as_ref().unwrap().clone();
                p = t;
            }
            p.borrow_mut().is_end = true;
        }
        stream_checker.root.borrow_mut().fail = Some(stream_checker.root.clone());
        let mut queue = {
            let mut queue = VecDeque::new();
            let mut root = stream_checker.root.borrow_mut();
            for i in 0..26 {
                if root.sons[i].is_some() {
                    let son = root.sons[i].as_ref().unwrap();
                    son.borrow_mut().fail = Some(stream_checker.root.clone());
                    queue.push_back(son.clone());
                } else {
                    root.sons[i] = Some(stream_checker.root.clone());
                }
            }
            queue
        };
        while let Some(node) = queue.pop_front() {
            let mut node_mut = node.borrow_mut();
            node_mut.is_end = node_mut.is_end || node_mut.fail.as_ref().unwrap().borrow().is_end;
            for i in 0..26 {
                if node_mut.sons[i].is_some() {
                    let son = node_mut.sons[i].as_ref().unwrap();
                    son.borrow_mut().fail =
                        node_mut.fail.as_ref().unwrap().borrow().sons[i].clone();
                    queue.push_back(son.clone());
                } else {
                    let t = node_mut.fail.as_ref().unwrap().borrow().sons[i].clone();
                    node_mut.sons[i] = t;
                }
            }
        }
        stream_checker.now = stream_checker.root.clone();
        stream_checker
    }

    pub fn query(&mut self, letter: char) -> bool {
        let t = self.now.borrow().sons[letter as usize - 'a' as usize]
            .as_ref()
            .unwrap()
            .clone();
        self.now = t;
        self.now.borrow().is_end
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        let mut stream_checker = StreamChecker::new(["cd", "f", "kl"].to_vec_string());
        assert!(!stream_checker.query('a'));
        assert!(!stream_checker.query('b'));
        assert!(!stream_checker.query('c'));
        assert!(stream_checker.query('d'));
        assert!(!stream_checker.query('e'));
        assert!(stream_checker.query('f'));
        assert!(!stream_checker.query('g'));
        assert!(!stream_checker.query('h'));
        assert!(!stream_checker.query('i'));
        assert!(!stream_checker.query('j'));
        assert!(!stream_checker.query('k'));
        assert!(stream_checker.query('l'));
    }
}
