pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min = 0;
        let mut sum: i32 = 0;
        for (i, (gas, cost)) in gas.into_iter().zip(cost).enumerate() {
            sum += gas - cost;
            if sum < min {
                min = sum;
                result = i as i32 + 1;
            }
        }
        if sum < 0 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_complete_circuit([1, 2, 3, 4, 5].to_vec(), [3, 4, 5, 1, 2].to_vec()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_complete_circuit([2, 3, 4].to_vec(), [3, 4, 3].to_vec()),
            -1
        );
    }
}
