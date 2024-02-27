pub struct Solution;

struct Edge {
    to: usize,
    next: i32,
}

impl Solution {
    pub fn count_paths(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        fn get_primes(primes: &mut [bool]) {
            for i in 0..primes.len() {
                if primes[i] {
                    let mut j = i * i;
                    while j < primes.len() {
                        primes[j] = false;
                        j += i;
                    }
                }
            }
        }

        fn add_edge(head: &mut [i32], edges: &mut Vec<Edge>, from: usize, to: usize) {
            let edge = Edge {
                to,
                next: head[from],
            };
            edges.push(edge);
            head[from] = edges.len() as i32 - 1;
        }

        fn count_paths_inner(
            primes: &[bool],
            head: &[i32],
            edges: &[Edge],
            pre: i32,
            idx: usize,
        ) -> (i64, i64, i64) {
            let mut cur = head[idx];
            let mut counts = Vec::new();
            let mut total = 0;
            while cur != -1 {
                let edge = &edges[cur as usize];
                if edge.to != pre as usize {
                    let (zero_count, one_count, total_count) =
                        count_paths_inner(primes, head, edges, idx as i32, edge.to);
                    total += total_count;
                    counts.push((zero_count, one_count));
                }
                cur = edge.next;
            }
            let zero = counts.iter().map(|(x, _)| x).sum::<i64>();
            let one = counts.iter().map(|(_, y)| y).sum::<i64>();
            if primes[idx + 1] {
                let mut delta = 0;
                for (i, _) in counts {
                    delta += (zero - i) * i;
                }
                total += delta / 2;
                (0, zero + 1, total + zero)
            } else {
                for (i, j) in counts {
                    total += i * (one - j);
                }
                (zero + 1, one, total + one)
            }
        }

        if n == 1 {
            return 0;
        }

        let n = n as usize;
        let mut primes = vec![true; n + 1];
        primes[0] = false;
        primes[1] = false;
        get_primes(&mut primes);

        let mut nodes = vec![-1; n];
        let mut edge_vec = Vec::with_capacity(n * 2);
        for edge in edges {
            let from = edge[0] as usize - 1;
            let to = edge[1] as usize - 1;
            add_edge(&mut nodes, &mut edge_vec, from, to);
            add_edge(&mut nodes, &mut edge_vec, to, from);
        }
        let idx = nodes.iter().position(|&x| x != -1).unwrap();
        let (_, _, result) = count_paths_inner(&primes, &nodes, &edge_vec, -1, idx);
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
            Solution::count_paths(5, [[1, 2], [1, 3], [2, 4], [2, 5]].to_vec_vec()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_paths(6, [[1, 2], [1, 3], [2, 4], [3, 5], [3, 6]].to_vec_vec()),
            6
        );
    }
}
