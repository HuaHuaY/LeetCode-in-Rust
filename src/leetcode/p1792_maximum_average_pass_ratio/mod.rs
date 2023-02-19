pub struct Solution;

use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Class {
    pass: i32,
    total: i32,
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let val = |pass: f64, total: f64| (pass + 1.0) / (total + 1.0) - pass / total;
        let self_val = val(self.pass as f64, self.total as f64);
        let other_val = val(other.pass as f64, other.total as f64);
        self_val.partial_cmp(&other_val).unwrap()
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let len = classes.len();
        let mut heap = BinaryHeap::new();
        for class in classes {
            heap.push(Class {
                pass: class[0],
                total: class[1],
            });
        }
        for _ in 0..extra_students {
            let mut class = heap.pop().unwrap();
            class.pass += 1;
            class.total += 1;
            heap.push(class);
        }
        (heap
            .into_iter()
            .map(|class| class.pass as f64 / class.total as f64)
            .sum::<f64>()
            / len as f64
            * 100_000.0)
            .round()
            / 100_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_average_ratio([[1, 2], [3, 5], [2, 2]].to_vec_vec(), 2),
            0.78333
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_average_ratio([[2, 4], [3, 9], [4, 5], [2, 10]].to_vec_vec(), 4),
            0.53485
        );
    }
}
