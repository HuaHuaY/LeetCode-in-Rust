pub struct Solution;

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let len = cars.len();
        let mut result = vec![0f64; len];
        *result.last_mut().unwrap() = -1f64;

        let mut stack = Vec::new();
        stack.push(len - 1);

        for (i, car) in cars.iter().enumerate().rev().skip(1) {
            while let Some(&last) = stack.last() {
                if car[1] <= cars[last][1]
                    || (result[last] != -1f64
                        && (cars[last][0] - car[0]) as f64 / (car[1] - cars[last][1]) as f64
                            >= result[last])
                {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(&last) = stack.last() {
                result[i] = (cars[last][0] - car[0]) as f64 / (car[1] - cars[last][1]) as f64;
            } else {
                result[i] = -1f64;
            }
            stack.push(i);
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
            Solution::get_collision_times([[1, 2], [2, 1], [4, 3], [7, 2]].to_vec_vec()),
            [1.00000, -1.00000, 3.00000, -1.00000]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_collision_times([[3, 4], [5, 4], [6, 3], [9, 1]].to_vec_vec()),
            [2.00000, 1.00000, 1.50000, -1.00000]
        );
    }
}
