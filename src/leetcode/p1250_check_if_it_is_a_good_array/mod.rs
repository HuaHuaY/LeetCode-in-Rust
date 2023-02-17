pub struct Solution;

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                std::mem::swap(&mut a, &mut b);
                b %= a;
            }
            a
        }

        let mut num = nums[0];
        for i in nums.into_iter().skip(1) {
            num = gcd(num, i);
            if num == 1 {
                return true;
            }
        }
        num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_good_array([12, 5, 7, 23].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(Solution::is_good_array([29, 6, 10].to_vec()));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_good_array([3, 6].to_vec()));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_good_array([1].to_vec()));
    }
}
