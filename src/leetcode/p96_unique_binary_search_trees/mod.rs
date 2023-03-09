pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        fn num_trees_inner(map: &mut HashMap<i32, i32>, left: i32, right: i32) -> i32 {
            if let Some(num) = map.get(&(right - left)) {
                return *num;
            }

            let mut count = 0;
            for i in left..=right {
                let left = if i > left {
                    num_trees_inner(map, left, i - 1)
                } else {
                    1
                };

                let right = if i < right {
                    num_trees_inner(map, i + 1, right)
                } else {
                    1
                };

                count += left * right;
            }
            map.insert(right - left, count);
            count
        }
        num_trees_inner(&mut HashMap::new(), 1, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
}
