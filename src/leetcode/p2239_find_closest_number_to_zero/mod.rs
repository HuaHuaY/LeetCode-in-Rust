pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((i32::MAX, i32::MIN), |(abs, num), i| {
                match i.abs().cmp(&abs) {
                    Ordering::Greater => (abs, num),
                    Ordering::Equal => (abs, num.max(i)),
                    Ordering::Less => (i.abs(), i),
                }
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_closest_number([-4, -2, 1, 4, 8].to_vec()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_closest_number([2, -1, 1].to_vec()), 1);
    }
}
