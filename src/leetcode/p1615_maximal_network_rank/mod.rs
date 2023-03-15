pub struct Solution;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let roads_len = roads.len();
        let mut count = vec![0; n];
        let mut connected = vec![vec![false; n]; n];
        for road in roads {
            let a = road[0] as usize;
            let b = road[1] as usize;
            count[a] += 1;
            count[b] += 1;
            connected[a][b] = true;
            connected[b][a] = true;
        }

        let mut first_count = 0;
        let mut second_count = 0;
        for count in &count {
            if *count > first_count {
                second_count = first_count;
                first_count = *count;
            } else if *count > second_count {
                second_count = *count;
            }
        }

        let mut first_count_city = vec![];
        let mut second_count_city = vec![];
        for (idx, count) in count.iter().enumerate() {
            if *count == first_count {
                first_count_city.push(idx);
            } else if *count == second_count {
                second_count_city.push(idx);
            }
        }

        let first_len = first_count_city.len();
        if first_len == 1 {
            if second_count_city
                .into_iter()
                .any(|idx| !connected[idx][first_count_city[0]])
            {
                second_count + first_count
            } else {
                second_count + first_count - 1
            }
        } else if first_len * (first_len - 1) / 2 > roads_len {
            first_count + second_count
        } else {
            for i in &first_count_city {
                for j in &first_count_city {
                    if *i != *j && !connected[*i][*j] {
                        return first_count * 2;
                    }
                }
            }
            first_count * 2 - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecVec;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximal_network_rank(4, [[0, 1], [0, 3], [1, 2], [1, 3]].to_vec_vec()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximal_network_rank(
                5,
                [[0, 1], [0, 3], [1, 2], [1, 3], [2, 3], [2, 4]].to_vec_vec()
            ),
            5
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximal_network_rank(
                8,
                [[0, 1], [1, 2], [2, 3], [2, 4], [5, 6], [5, 7]].to_vec_vec()
            ),
            5
        );
    }
}
