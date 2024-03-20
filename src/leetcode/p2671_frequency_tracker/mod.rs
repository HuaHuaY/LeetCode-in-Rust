pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
struct FrequencyTracker {
    nums: HashMap<i32, i32>,
    freq: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl FrequencyTracker {
    fn new() -> Self {
        FrequencyTracker::default()
    }

    fn add(&mut self, number: i32) {
        let val = self.nums.entry(number).or_insert(0);
        *self.freq.entry(*val).or_insert(0) -= 1;
        *val += 1;
        *self.freq.entry(*val).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if let Entry::Occupied(entry) = self.nums.entry(number) {
            let val = entry.into_mut();
            *self.freq.entry(*val).or_insert(0) -= 1;
            *val = 0.max(*val - 1);
            *self.freq.entry(*val).or_insert(0) += 1;
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        if let Some(x) = self.freq.get(&frequency) {
            return *x > 0;
        }
        false
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = FrequencyTracker::new();
        obj.add(3);
        obj.add(3);
        assert!(obj.has_frequency(2))
    }

    #[test]
    fn test2() {
        let mut obj = FrequencyTracker::new();
        obj.add(1);
        obj.delete_one(1);
        assert!(!obj.has_frequency(1))
    }
}
