pub struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut pre_sum = vec![0; nums.len()];
        pre_sum[0] = nums[0];
        for (i, num) in nums.into_iter().enumerate().skip(1) {
            pre_sum[i] = pre_sum[i - 1] + num;
        }
        let mut answer = vec![0; queries.len()];
        for (i, query) in queries.into_iter().enumerate() {
            match pre_sum.binary_search(&query) {
                Ok(idx) => answer[i] = idx as i32 + 1,
                Err(idx) => answer[i] = idx as i32,
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::answer_queries([4, 5, 2, 1].to_vec(), [3, 10, 21].to_vec()),
            [2, 3, 4]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::answer_queries([2, 3, 4, 5].to_vec(), [1].to_vec()),
            [0]
        );
    }
}
