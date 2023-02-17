pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .fold(vec![0; 101], |mut vec, i| {
                vec[i as usize] += 1;
                vec
            })
            .into_iter()
            .fold(vec![0, 0], |mut result, i| {
                result[0] += i >> 1;
                result[1] += i & 1;
                result
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_pairs([1, 3, 2, 1, 3, 2, 2].to_vec()),
            [3, 1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::number_of_pairs([1, 1].to_vec()), [1, 0]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::number_of_pairs([0].to_vec()), [0, 1]);
    }
}
