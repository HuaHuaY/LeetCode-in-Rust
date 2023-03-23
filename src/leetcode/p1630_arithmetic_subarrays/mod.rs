pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(l.len());
        'a: for (left, right) in l.into_iter().zip(r) {
            let nums = &nums[left as usize..=right as usize];
            let len = right - left + 1;
            let max = *nums.iter().max().unwrap();
            let min = *nums.iter().min().unwrap();
            if min == max {
                result.push(true);
            } else if (max - min) % (len - 1) != 0 {
                result.push(false);
            } else {
                let diff = (max - min) / (len - 1);
                let mut bitmap = vec![false; len as usize];
                for j in nums {
                    if (*j - min) % diff != 0 {
                        result.push(false);
                        continue 'a;
                    }
                    let d = (*j - min) / diff;
                    if bitmap[d as usize] {
                        result.push(false);
                        continue 'a;
                    } else {
                        bitmap[d as usize] = true;
                    }
                }
                result.push(true);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                [4, 6, 5, 9, 3, 7].to_vec(),
                [0, 0, 2].to_vec(),
                [2, 3, 5].to_vec()
            ),
            [true, false, true]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::check_arithmetic_subarrays(
                [-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10].to_vec(),
                [0, 1, 6, 4, 8, 7].to_vec(),
                [4, 4, 9, 7, 9, 10].to_vec()
            ),
            [false, true, false, false, true, true]
        );
    }
}
