pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_candies(
        mut status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let mut result = 0;

        let mut boxes = HashSet::new();
        let mut open_queue = Vec::new();
        for b in initial_boxes {
            if status[b as usize] == 1 {
                open_queue.push(b as usize);
            }
            boxes.insert(b as usize);
        }

        while !open_queue.is_empty() {
            while let Some(b) = open_queue.pop() {
                boxes.remove(&b);
                result += candies[b];
                for &k in &keys[b] {
                    status[k as usize] = 1;
                }
                for inner_box in contained_boxes[b].iter().map(|&i| i as usize) {
                    boxes.insert(inner_box);
                }
            }
            for &close_box in &boxes {
                if status[close_box] == 1 {
                    open_queue.push(close_box);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_candies(
                [1, 0, 1, 0].to_vec(),
                [7, 5, 4, 100].to_vec(),
                vec_vec![[], [], [1], []],
                vec_vec![[1, 2], [3], [], []],
                [0].to_vec(),
            ),
            16
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_candies(
                [1, 0, 0, 0, 0, 0].to_vec(),
                [1, 1, 1, 1, 1, 1].to_vec(),
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                vec_vec![[1, 2, 3, 4, 5], [], [], [], [], []],
                [0].to_vec(),
            ),
            6
        );
    }
}
