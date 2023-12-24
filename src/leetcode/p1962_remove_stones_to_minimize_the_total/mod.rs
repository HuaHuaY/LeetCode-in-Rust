pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for pile in &piles {
            heap.push(*pile);
        }

        for _ in 0..k {
            let pile = heap.pop().unwrap();
            heap.push(pile - pile / 2);
        }

        heap.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_stone_sum([5, 4, 9].to_vec(), 2), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_stone_sum([4, 3, 6, 7].to_vec(), 3), 12);
    }
}
