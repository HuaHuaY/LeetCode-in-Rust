pub struct Solution;

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let tmp = b;
                b = a.rem_euclid(b);
                a = tmp;
            }
            a
        }

        target_capacity <= jug1_capacity + jug2_capacity
            && target_capacity.rem_euclid(gcd(jug1_capacity, jug2_capacity)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_measure_water(3, 5, 4));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_measure_water(2, 6, 5));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_measure_water(1, 2, 3));
    }
}
