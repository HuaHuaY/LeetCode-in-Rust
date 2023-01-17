pub struct Solution;

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let max_num = *nums.iter().max().unwrap() as usize;
        let mut exist_num = vec![false; max_num + 1];
        for i in nums {
            exist_num[i as usize] = true;
        }
        let mut ans = 0;
        for i in 1..=max_num {
            let mut tmp = 0;
            for j in (i..=max_num).step_by(i) {
                if exist_num[j] {
                    if tmp == 0 {
                        tmp = j;
                    } else {
                        tmp = gcd(tmp, j);
                    }

                    if tmp == i {
                        ans += 1;
                        break;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_different_subsequence_gc_ds(vec![6, 10, 3]),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_different_subsequence_gc_ds(vec![5, 15, 40, 5, 6]),
            7
        );
    }
}
