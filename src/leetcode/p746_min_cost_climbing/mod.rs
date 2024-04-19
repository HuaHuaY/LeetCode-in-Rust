pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut a = cost[0];
        let mut b = cost[1];
        for c in cost.into_iter().skip(2) {
            std::mem::swap(&mut a, &mut b);
            b = a.min(b) + c;
        }
        a.min(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_cost_climbing_stairs([10, 15, 20].to_vec()),
            15
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1].to_vec()),
            6
        );
    }
}
