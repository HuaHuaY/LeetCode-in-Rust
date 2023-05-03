pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut result = HashSet::new();
        let mut x_del = 1;
        for _ in 0..21 {
            let mut y_del = 1;
            for _ in 0..21 {
                if x_del + y_del <= bound {
                    result.insert(x_del + y_del);
                    y_del *= y;
                } else {
                    break;
                }
            }
            x_del *= x;
            if x_del > bound {
                break;
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::powerful_integers(2, 3, 10);
        result.sort_unstable();
        let mut answer = [2, 3, 4, 5, 7, 9, 10].to_vec();
        answer.sort_unstable();
        assert_eq!(result, answer);
    }

    #[test]
    fn test2() {
        let mut result = Solution::powerful_integers(3, 5, 15);
        result.sort_unstable();
        let mut answer = [2, 4, 6, 8, 10, 14].to_vec();
        answer.sort_unstable();
        assert_eq!(result, answer);
    }
}
