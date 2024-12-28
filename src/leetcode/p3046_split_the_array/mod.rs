pub struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        !nums
            .into_iter()
            .fold([0; 100], |mut acc, x| {
                acc[x as usize - 1] += 1;
                acc
            })
            .into_iter()
            .any(|i| i > 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_possible_to_split([1, 1, 2, 2, 3, 4].to_vec()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_possible_to_split([1, 1, 1, 1].to_vec()));
    }
}
