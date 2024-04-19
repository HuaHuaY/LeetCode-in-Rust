pub struct Solution;

impl Solution {
    pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut arr = vec![(nums[0], nums[0])];
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                arr.last_mut().unwrap().1 += nums[i];
            } else {
                arr.push((nums[i], nums[i]));
            }
        }

        if arr.len() == 1 {
            return arr[0].1;
        }

        let mut a = (0, 0);
        let mut b = arr[0];
        for c in arr.into_iter().skip(1) {
            std::mem::swap(&mut a, &mut b);
            if c.0 == a.0 + 1 {
                b = (c.0, a.1.max(b.1 + c.1))
            } else {
                b = (c.0, c.1 + a.1);
            }
        }
        b.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::delete_and_earn([3, 4, 2].to_vec()), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::delete_and_earn([2, 2, 3, 3, 3, 4].to_vec()), 9);
    }
}
