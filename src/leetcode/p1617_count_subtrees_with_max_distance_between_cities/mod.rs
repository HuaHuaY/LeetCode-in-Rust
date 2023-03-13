pub struct Solution;

impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(mask: &mut i32, diameter: &mut usize, root: usize, nodes: &[Vec<usize>]) -> usize {
            let mut max1 = 0;
            let mut max2 = 0;
            *mask &= !(1 << root);
            for vertex in &nodes[root] {
                if *mask & (1 << vertex) != 0 {
                    *mask &= !(1 << vertex);
                    let d = 1 + dfs(mask, diameter, *vertex, nodes);
                    if d > max1 {
                        max2 = max1;
                        max1 = d;
                    } else if d > max2 {
                        max2 = d;
                    }
                }
            }
            *diameter = (*diameter).max(max1 + max2);
            max1
        }

        let mut nodes = vec![vec![]; n as usize];
        for edge in edges {
            let a = (edge[0] - 1) as usize;
            let b = (edge[1] - 1) as usize;
            nodes[a].push(b);
            nodes[b].push(a);
        }

        let mut result = vec![0; n as usize - 1];
        for i in 1..(1 << n) {
            let root = 32 - (i as u32).leading_zeros() as usize - 1;
            let mut mask = i;
            let mut diameter = 0;
            dfs(&mut mask, &mut diameter, root, &nodes);
            if mask == 0 && diameter > 0 {
                result[diameter - 1] += 1;
            }
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
            Solution::count_subgraphs_for_each_diameter(4, [[1, 2], [2, 3], [2, 4]].to_vec_vec()),
            [3, 4, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(2, [[1, 2]].to_vec_vec()),
            [1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_subgraphs_for_each_diameter(3, [[1, 2], [2, 3]].to_vec_vec()),
            [2, 1]
        );
    }
}
