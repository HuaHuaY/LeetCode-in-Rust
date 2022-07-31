pub struct Solution {}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (start, destination) = if start < destination {
            (start, destination)
        } else {
            (destination, start)
        };

        let distance = vec![0]
            .into_iter()
            .chain(distance.into_iter())
            .scan(0, |state, x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<i32>>();
        let dis = distance[destination as usize] - distance[start as usize];
        let dis_r = *distance.last().unwrap() - dis;
        dis.min(dis_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
            4
        );
    }
}
