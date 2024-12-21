pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn power(cache: &mut HashMap<i32, i32>, x: i32) -> i32 {
        if let Some(&v) = cache.get(&x) {
            v
        } else {
            let t = Solution::power(cache, if x & 1 == 0 { x / 2 } else { 3 * x + 1 }) + 1;
            cache.insert(x, t);
            t
        }
    }

    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut cache = HashMap::with_capacity((hi - lo + 1) as usize);
        cache.insert(1, 0);
        let mut vec = (lo..=hi).collect::<Vec<_>>();
        vec.sort_by_cached_key(|&i| ((Solution::power(&mut cache, i) as i64) << 32) | (i as i64));
        vec[k as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
