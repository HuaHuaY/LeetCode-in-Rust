pub struct Solution {}

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            while n % 2 == 0 {
                n >>= 1;
            }
            while n % 3 == 0 {
                n /= 3;
            }
            while n % 5 == 0 {
                n /= 5;
            }
            return n == 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::is_ugly(6), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_ugly(8), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_ugly(14), false);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::is_ugly(1), true);
    }
}
