pub struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = Vec::new();
        let mut cnt: Vec<Vec<i32>> = Vec::new();
        for num in nums {
            let i = match dp.binary_search_by(|d| d.last().unwrap().cmp(&num)) {
                Ok(j) => j,
                Err(j) => j,
            };
            let mut c = 1;
            if i > 0 {
                let k = match dp[i - 1].binary_search_by(|d| num.cmp(d)) {
                    Ok(mut j) => {
                        while j + 1 < dp[i - 1].len() && dp[i - 1][j] == dp[i - 1][j + 1] {
                            j += 1;
                        }
                        j + 1
                    }
                    Err(j) => j,
                };
                c = *cnt[i - 1].last().unwrap() - cnt[i - 1][k];
            }
            if i == dp.len() {
                dp.push(vec![num]);
                cnt.push(vec![0, c]);
            } else {
                dp[i].push(num);
                let last = *cnt[i].last().unwrap();
                cnt[i].push(last + c);
            }
        }
        *cnt.last().unwrap().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_number_of_lis([1, 3, 5, 4, 7].to_vec()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_number_of_lis([2, 2, 2, 2, 2].to_vec()), 5);
    }
}
