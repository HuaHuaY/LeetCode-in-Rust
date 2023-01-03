pub struct Solution;

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let length = a.len();
        if length < 2 {
            true
        } else {
            let mut i = 0;
            let mut is_increasing = true;
            while i < length - 1 {
                if a[i] <= a[i + 1] {
                    i += 1;
                    continue;
                } else {
                    is_increasing = false;
                    break;
                }
            }
            i = 0;
            let mut is_decreasing = true;
            while i < length - 1 {
                if a[i] >= a[i + 1] {
                    i += 1;
                    continue;
                } else {
                    is_decreasing = false;
                    break;
                }
            }

            is_increasing || is_decreasing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
    }

    #[test]
    fn test5() {
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
    }
}
