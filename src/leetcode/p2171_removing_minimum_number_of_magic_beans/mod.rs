pub struct Solution;

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let len = beans.len();
        let sum = beans.iter().map(|i| *i as i64).sum::<i64>();
        let beans = beans.into_iter().fold(vec![0; 100001], |mut arr, i| {
            arr[i as usize] += 1;
            arr
        });

        let mut result = i64::MIN;
        let mut left_count = 0;
        for (i, bean) in IntoIterator::into_iter(beans)
            .enumerate()
            .filter(|(_, v)| *v > 0)
        {
            result = result.max((len - left_count) as i64 * i as i64);
            left_count += bean;
        }
        sum - result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_removal([4, 1, 6, 5].to_vec()), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_removal([2, 10, 3, 2].to_vec()), 7);
    }
}
