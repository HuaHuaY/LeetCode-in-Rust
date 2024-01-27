pub struct Solution;

impl Solution {
    pub fn max_number_of_alloys(
        _: i32,
        _: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        fn valid(composition: &[i32], stock: &[i32], cost: &[i32], budget: i32, mid: i32) -> bool {
            let mut total = 0;
            for ((&c, &s), &p) in composition.iter().zip(stock).zip(cost) {
                let diff = c as i64 * mid as i64 - s as i64;
                if diff > 0 {
                    total += diff * p as i64;
                }
            }
            total <= budget as i64
        }

        let mut result = 0;
        for composition in composition {
            let mut left = 0;
            let mut right = 2 * 100_000_000;
            let mut mid;
            while left <= right {
                mid = (right - left) / 2 + left;
                if valid(&composition, &stock, &cost, budget, mid) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            result = result.max(right);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_number_of_alloys(
                3,
                2,
                15,
                [[1, 1, 1], [1, 1, 10]].to_vec_vec(),
                [0, 0, 0].to_vec(),
                [1, 2, 3].to_vec()
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_number_of_alloys(
                3,
                2,
                15,
                [[1, 1, 1], [1, 1, 10]].to_vec_vec(),
                [0, 0, 100].to_vec(),
                [1, 2, 3].to_vec()
            ),
            5
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_number_of_alloys(
                2,
                3,
                10,
                [[2, 1], [1, 2], [1, 1]].to_vec_vec(),
                [1, 1].to_vec(),
                [5, 5].to_vec()
            ),
            2
        );
    }
}
