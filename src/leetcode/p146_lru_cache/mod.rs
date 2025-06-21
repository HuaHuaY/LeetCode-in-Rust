pub struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    key: i32,
    value: i32,
    prev: Weak<RefCell<Node>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for Node {}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Option<Rc<RefCell<Node>>>,
    tail_key: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail_key: 0,
        }
    }

    fn move_head(&mut self, node: Rc<RefCell<Node>>) -> i32 {
        let mut n = node.borrow_mut();
        if self.tail_key != n.key {
            let next = n.next.as_ref().unwrap();
            let mut next = next.borrow_mut();
            next.prev = n.prev.clone();
        }
        if let Some(prev) = n.prev.upgrade() {
            let mut prev = prev.borrow_mut();
            if self.tail_key == n.key {
                self.tail_key = prev.key;
            }
            prev.next = n.next.clone();
        }
        if let Some(head) = &self.head {
            let mut head = head.borrow_mut();
            head.prev = Rc::downgrade(&node);
        }
        n.next = self.head.take();
        n.prev = Weak::new();
        let v = n.value;
        self.head = Some(node.clone());
        v
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key).cloned() {
            if &node != self.head.as_ref().unwrap() {
                return self.move_head(node);
            }
            return node.borrow().value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            if &node != self.head.as_ref().unwrap() {
                self.move_head(node.clone());
            }
            node.borrow_mut().value = value;
            return;
        }

        let len = self.map.len();
        if len == self.capacity {
            if len == 1 {
                self.head = None;
            }
            let tail_node = self.map.remove(&self.tail_key).unwrap();
            let tail_node = tail_node.borrow_mut();
            if let Some(prev) = tail_node.prev.upgrade() {
                let mut prev = prev.borrow_mut();
                prev.next = None;
                self.tail_key = prev.key;
            }
        }

        if self.map.is_empty() {
            self.tail_key = key;
        }
        let node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: Weak::new(),
            next: self.head.take(),
        }));
        {
            let n = node.borrow();
            if let Some(head) = &n.next {
                let mut head = head.borrow_mut();
                head.prev = Rc::downgrade(&node);
            }
        }
        self.head = Some(node.clone());
        self.map.insert(key, node);
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }

    #[test]
    fn test2() {
        let mut lru_cache = LRUCache::new(1);
        lru_cache.put(2, 1);
        assert_eq!(lru_cache.get(2), 1);
    }

    #[test]
    fn test3() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(2, 1);
        lru_cache.put(3, 2);
        assert_eq!(lru_cache.get(3), 2);
        assert_eq!(lru_cache.get(2), 1);
        lru_cache.put(4, 3);
        assert_eq!(lru_cache.get(2), 1);
        assert_eq!(lru_cache.get(3), -1);
        assert_eq!(lru_cache.get(4), 3);
    }
}
