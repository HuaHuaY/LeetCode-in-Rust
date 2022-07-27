pub struct MyHashMap {
    capacity: usize,
    map: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        let capacity = 997;
        let map = vec![Vec::new(); capacity];
        MyHashMap { capacity, map }
    }

    /** value will always be non-negative. */
    pub fn put(&mut self, key: i32, value: i32) {
        let index = key as usize % self.capacity;
        if let Some((_, j)) = self.map[index].iter_mut().find(|x| x.0 == key) {
            *j = value;
        } else {
            self.map[index].push((key, value));
        }
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        let index = key as usize % self.capacity;
        if let Some((_, j)) = self.map[index].iter().find(|x| x.0 == key) {
            *j
        } else {
            -1
        }
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    pub fn remove(&mut self, key: i32) {
        let index = key as usize % self.capacity;
        self.map[index].retain(|&x| x.0 != key);
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut test = MyHashMap::new();
        test.put(1, 1);
        test.put(2, 2);

        assert_eq!(test.get(1), 1);
        assert_eq!(test.get(3), -1);

        test.put(2, 1);
        assert_eq!(test.get(2), 1);
        
        test.remove(2);
        assert_eq!(test.get(2), -1);
    }
}
