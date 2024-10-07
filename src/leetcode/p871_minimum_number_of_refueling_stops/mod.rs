pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        if start_fuel >= target {
            return 0;
        }

        let mut queue = BinaryHeap::new();
        let mut result = 0;
        let mut fuel = start_fuel;

        for station in stations {
            if fuel >= station[0] {
                queue.push(station[1]);
            } else {
                while let Some(f) = queue.pop() {
                    fuel += f;
                    result += 1;
                    if fuel >= target {
                        return result;
                    } else if fuel >= station[0] {
                        queue.push(station[1]);
                        break;
                    }
                }
                if fuel < station[0] {
                    return -1;
                }
            }
        }
        while let Some(f) = queue.pop() {
            fuel += f;
            result += 1;
            if fuel >= target {
                return result;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_refuel_stops(1, 1, [[]; 0].to_vec_vec()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_refuel_stops(100, 1, [[10, 100]].to_vec_vec()),
            -1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_refuel_stops(
                100,
                10,
                [[10, 60], [20, 30], [30, 30], [60, 40]].to_vec_vec()
            ),
            2
        );
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::min_refuel_stops(100, 1, [[]; 0].to_vec_vec()), -1);
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::min_refuel_stops(
                1000000000,
                1000000000,
                [[5, 1000000000], [1000, 1000000000], [100000, 1000000000]].to_vec_vec()
            ),
            0
        );
    }
}
