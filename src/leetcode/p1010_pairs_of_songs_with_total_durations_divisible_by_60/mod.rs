pub struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut slots = [0; 60];
        for t in time {
            let t = t as usize % 60;
            count += slots[(60 - t) % 60];
            slots[t] += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_pairs_divisible_by60([30, 20, 150, 100, 40].to_vec()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_pairs_divisible_by60([60, 60, 60].to_vec()), 3);
    }
}
