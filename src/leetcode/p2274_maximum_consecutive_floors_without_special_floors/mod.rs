pub struct Solution;

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort_unstable();
        special
            .into_iter()
            .chain(std::iter::once(top + 1))
            .fold((bottom - 1, 0), |(pre, max), i| (i, max.max(i - pre - 1)))
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_consecutive(2, 9, [4, 6].to_vec()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_consecutive(6, 8, [7, 6, 8].to_vec()), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_consecutive(3, 15, [7, 9, 13].to_vec()), 4);
    }
}
