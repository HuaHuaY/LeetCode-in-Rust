pub struct Solution;

impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort_unstable();
        let length = stones.len();
        if stones[length - 1] - stones[0] + 1 - length as i32 == 0 {
            return vec![0, 0];
        }
        let max = (stones[length - 1] - stones[1]).max(stones[length - 2] - stones[0]) + 2
            - length as i32;
        let mut min = i32::MAX;
        if stones[length - 1] - stones[1] == length as i32 - 2 {
            min = if stones[1] - stones[0] == 2 { 1 } else { 2 };
            return vec![min, max];
        } else if stones[length - 2] - stones[0] == length as i32 - 2 {
            min = if stones[length - 1] - stones[length - 2] == 2 {
                1
            } else {
                2
            };
            return vec![min, max];
        }
        let mut j = 0;
        for (i, stone) in stones.iter().enumerate() {
            while j < length && stones[j] - *stone < length as i32 {
                j += 1;
            }
            min = min.min((length - (j - i)) as i32);
            if j == length {
                break;
            }
        }
        vec![min, max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_moves_stones_ii([7, 4, 9].to_vec()), [1, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_moves_stones_ii([6, 5, 4, 3, 10].to_vec()),
            [2, 3]
        );
    }
}
