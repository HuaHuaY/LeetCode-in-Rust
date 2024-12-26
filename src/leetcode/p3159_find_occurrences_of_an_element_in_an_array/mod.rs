pub struct Solution;

impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, mut queries: Vec<i32>, x: i32) -> Vec<i32> {
        let vec = nums
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b == x)
            .map(|(a, _)| a as i32)
            .collect::<Vec<_>>();
        queries
            .iter_mut()
            .for_each(|i| *i = vec.get((*i) as usize - 1).copied().unwrap_or(-1));
        queries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::occurrences_of_element([1, 3, 1, 7].to_vec(), [1, 3, 2, 4].to_vec(), 1),
            [0, -1, 2, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::occurrences_of_element([1, 2, 3].to_vec(), [10].to_vec(), 5),
            [-1]
        );
    }
}
