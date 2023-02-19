pub struct Solution;

use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
};

pub struct AuthenticationManager {
    time_to_live: i32,
    min_heap: BinaryHeap<Reverse<(i32, String)>>,
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AuthenticationManager {
    pub fn new(time_to_live: i32) -> Self {
        Self {
            time_to_live,
            min_heap: BinaryHeap::new(),
            map: HashMap::new(),
        }
    }

    pub fn generate(&mut self, token_id: String, current_time: i32) {
        if let Entry::Vacant(entry) = self.map.entry(token_id.clone()) {
            let expired_time = current_time + self.time_to_live;
            entry.insert(expired_time);
            self.min_heap.push(Reverse((expired_time, token_id)));
        }
    }

    pub fn renew(&mut self, token_id: String, current_time: i32) {
        if let Entry::Occupied(mut entry) = self.map.entry(token_id) {
            let v = entry.get_mut();
            if *v > current_time {
                *v = current_time + self.time_to_live;
            }
        }
    }

    pub fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
        while let Some(&Reverse((expired_time, _))) = self.min_heap.peek() {
            if expired_time <= current_time {
                let Reverse((_, token_id)) = self.min_heap.pop().unwrap();
                let t = *self.map.get(&token_id).unwrap();
                if t > current_time {
                    self.min_heap.push(Reverse((t, token_id)));
                }
            } else {
                break;
            }
        }
        self.min_heap.len() as i32
    }
}

/**
 * Your AuthenticationManager object will be instantiated and called as such:
 * let obj = AuthenticationManager::new(timeToLive);
 * obj.generate(tokenId, currentTime);
 * obj.renew(tokenId, currentTime);
 * let ret_3: i32 = obj.count_unexpired_tokens(currentTime);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = AuthenticationManager::new(5);
        obj.renew("aaa".to_string(), 1);
        obj.generate("aaa".to_string(), 2);
        assert_eq!(1, obj.count_unexpired_tokens(6));
        obj.generate("bbb".to_string(), 7);
        obj.renew("aaa".to_string(), 8);
        obj.renew("bbb".to_string(), 10);
        assert_eq!(0, obj.count_unexpired_tokens(15));
    }
}
