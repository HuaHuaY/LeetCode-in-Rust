pub struct Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut i = 0;
        let mut j = 100000;
        while i <= j {
            let mid = i + (j - i) / 2;
            if 2 * mid * (mid + 1) * (2 * mid + 1) < needed_apples {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }
        i * 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_perimeter(1), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_perimeter(13), 16);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
    }
}
