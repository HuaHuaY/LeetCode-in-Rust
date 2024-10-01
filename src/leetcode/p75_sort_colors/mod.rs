pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let mut k = nums.len() as i32 - 1;
        while j as i32 <= k {
            match nums[j] {
                0 => {
                    nums.swap(i, j);
                    i += 1;
                    j += 1;
                }
                1 => {
                    j += 1;
                }
                2 => {
                    nums.swap(j, k as usize);
                    k -= 1;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut vec = [2, 0, 2, 1, 1, 0].to_vec();
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test2() {
        let mut vec = [2, 0, 1].to_vec();
        Solution::sort_colors(&mut vec);
        assert_eq!(vec, [0, 1, 2]);
    }
}
