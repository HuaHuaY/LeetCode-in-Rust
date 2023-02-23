pub struct Solution;

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(1 << n);
        res.push(0);
        for i in 1..=n {
            res.extend(res.clone().into_iter().rev().map(|x| x | (1 << (i - 1))));
        }
        let idx = res.iter().position(|x| *x == start).unwrap();
        res.rotate_left(idx);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::circular_permutation(2, 3), [3, 2, 0, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::circular_permutation(3, 2),
            [2, 6, 7, 5, 4, 0, 1, 3]
        );
    }
}
