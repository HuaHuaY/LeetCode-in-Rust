pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        nums1
            .into_iter()
            .map(|x| nums2.iter().filter(|&&y| (x % (y * k)) == 0).count() as i32)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_pairs([1, 3, 4].to_vec(), [1, 3, 4].to_vec(), 1),
            5
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_pairs([1, 2, 4, 12].to_vec(), [2, 4].to_vec(), 3),
            2
        );
    }
}
