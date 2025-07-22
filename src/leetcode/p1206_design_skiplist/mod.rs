pub struct Solution;

use std::cmp::Ordering;
use std::rc::Rc;
use std::sync::Mutex;

const MAX_LEVEL: usize = 16;

struct Node {
    value: i32,
    next: Vec<Option<Rc<Mutex<Node>>>>,
}

impl Node {
    fn new(value: i32, level: usize) -> Self {
        Node {
            value,
            next: vec![None; level],
        }
    }
}

struct Skiplist {
    head: Rc<Mutex<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Skiplist {
    fn new() -> Self {
        Skiplist {
            head: Rc::new(Mutex::new(Node::new(0, MAX_LEVEL))),
        }
    }

    fn search(&self, target: i32) -> bool {
        let mut current = self.head.clone();
        for level in (0..MAX_LEVEL).rev() {
            while let Some(next_node) = current.clone().lock().unwrap().next[level].clone() {
                match next_node.clone().lock().unwrap().value.cmp(&target) {
                    Ordering::Less => current = next_node,
                    Ordering::Equal => return true,
                    Ordering::Greater => break,
                }
            }
        }
        false
    }

    fn add(&mut self, num: i32) {
        let random_level = rand::random::<u64>() as usize % MAX_LEVEL;
        let new_node = Rc::new(Mutex::new(Node::new(num, random_level + 1)));

        let node_ref = new_node.clone();
        let mut node_ref = node_ref.lock().unwrap();

        let mut current = self.head.clone();
        for level in (0..MAX_LEVEL).rev() {
            while let Some(next_node) = current.clone().lock().unwrap().next[level].clone() {
                if next_node.lock().unwrap().value < num {
                    current = next_node;
                } else {
                    break;
                }
            }
            if level <= random_level {
                let mut guard = current.lock().unwrap();
                node_ref.next[level] = guard.next[level].clone();
                guard.next[level] = Some(new_node.clone());
            }
        }
    }

    fn erase(&mut self, num: i32) -> bool {
        let mut current = self.head.clone();
        let mut found = false;

        for level in (0..MAX_LEVEL).rev() {
            while let Some(next_node) = current.clone().lock().unwrap().next[level].clone() {
                match next_node.clone().lock().unwrap().value.cmp(&num) {
                    Ordering::Less => current = next_node,
                    Ordering::Equal => {
                        found = true;
                        break;
                    }
                    Ordering::Greater => break,
                }
            }
            if found {
                let mut guard = current.lock().unwrap();
                let next_node = guard.next[level].clone().unwrap();
                let sub_guard = next_node.lock().unwrap();
                guard.next[level] = sub_guard.next[level].clone();
            }
        }

        found
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * let obj = Skiplist::new();
 * let ret_1: bool = obj.search(target);
 * obj.add(num);
 * let ret_3: bool = obj.erase(num);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut skiplist = Skiplist::new();
        skiplist.add(1);
        skiplist.add(2);
        skiplist.add(3);
        assert!(!skiplist.search(0));
        skiplist.add(4);
        assert!(skiplist.search(1));
        assert!(!skiplist.erase(0));
        assert!(skiplist.erase(1));
        assert!(!skiplist.search(1));
    }

    #[test]
    fn test2() {
        let mut skiplist = Skiplist::new();
        skiplist.add(0);
        skiplist.add(5);
        skiplist.add(2);
        skiplist.add(1);
        assert!(skiplist.search(0));
        assert!(skiplist.erase(5));
        assert!(skiplist.search(2));
        assert!(!skiplist.search(3));
        assert!(skiplist.search(2));
    }
}
