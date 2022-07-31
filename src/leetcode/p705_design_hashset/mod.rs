pub struct MyHashSet {
    capacity: usize,
    set: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        let capacity = 997;
        let set = vec![Vec::new(); capacity];
        MyHashSet { set, capacity }
    }

    pub fn add(&mut self, key: i32) {
        self.set[key as usize % self.capacity].push(key);
    }

    pub fn remove(&mut self, key: i32) {
        self.set[key as usize % self.capacity].retain(|&x| x != key);
    }

    /** Returns true if this set contains the specified element */
    pub fn contains(&self, key: i32) -> bool {
        self.set[key as usize % self.capacity].contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut test = MyHashSet::new();
        test.add(1);
        test.add(2);

        assert_eq!(test.contains(1), true);
        assert_eq!(test.contains(3), false);

        test.add(2);
        assert_eq!(test.contains(2), true);

        test.remove(2);
        assert_eq!(test.contains(2), false);
    }
}
