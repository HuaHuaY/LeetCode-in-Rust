pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        candidates
            .into_iter()
            .fold([0; 24], |mut acc, mut i| {
                while i != 0 {
                    acc[i.trailing_zeros() as usize] += 1;
                    i &= i - 1;
                }
                acc
            })
            .into_iter()
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_combination([16, 17, 71, 62, 12, 24, 14].to_vec()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_combination([8, 8].to_vec()), 2);
    }
}
