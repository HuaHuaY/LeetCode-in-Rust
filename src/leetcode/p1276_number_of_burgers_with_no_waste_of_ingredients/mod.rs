pub struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let x = (tomato_slices - cheese_slices * 2) / 2;
        let y = (4 * cheese_slices - tomato_slices) / 2;
        if x >= 0 && y >= 0 && 4 * x + 2 * y == tomato_slices && x + y == cheese_slices {
            vec![x, y]
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_of_burgers(16, 7), [1, 6]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_of_burgers(17, 4), []);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_of_burgers(4, 17), []);
    }
}
