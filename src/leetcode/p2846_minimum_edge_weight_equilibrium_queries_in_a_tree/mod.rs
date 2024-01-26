pub struct Solution;

struct Tree {
    depth: Vec<i32>,
    pa: Vec<Vec<i32>>,
    cnt: Vec<Vec<[i32; 26]>>,
}

impl Tree {
    fn new(n: usize, edges: Vec<Vec<i32>>) -> Self {
        let m = n.ilog2() as usize + 1;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push((edge[1], edge[2] - 1));
            g[edge[1] as usize].push((edge[0], edge[2] - 1));
        }

        let mut depth = vec![0; n];
        let mut pa = vec![vec![-1; m]; n];
        let mut cnt = vec![vec![[0; 26]; m]; n];
        fn dfs(
            g: &[Vec<(i32, i32)>],
            depth: &mut [i32],
            pa: &mut [Vec<i32>],
            cnt: &mut [Vec<[i32; 26]>],
            x: usize,
            father: i32,
        ) {
            pa[x][0] = father;
            for (y, w) in &g[x] {
                if *y != father {
                    cnt[*y as usize][0][*w as usize] = 1;
                    depth[*y as usize] = depth[x] + 1;
                    dfs(g, depth, pa, cnt, *y as usize, x as i32);
                }
            }
        }
        dfs(&g, &mut depth, &mut pa, &mut cnt, 0, -1);

        for j in 0..m - 1 {
            for i in 0..n {
                if pa[i][j] != -1 {
                    pa[i][j + 1] = pa[pa[i][j] as usize][j];
                    for k in 0..26 {
                        cnt[i][j + 1][k] = cnt[i][j][k] + cnt[pa[i][j] as usize][j][k];
                    }
                }
            }
        }

        Tree { depth, pa, cnt }
    }

    fn get_kth_ancestor(&self, mut node: i32, mut k: i32) -> (i32, [i32; 26]) {
        let mut cw = [0; 26];
        while k != 0 && node != -1 {
            let trailing_zeros = k.trailing_zeros() as usize;
            for (i, item) in cw.iter_mut().enumerate() {
                *item += self.cnt[node as usize][trailing_zeros][i];
            }
            node = self.pa[node as usize][trailing_zeros];
            k &= k - 1;
        }
        (node, cw)
    }

    fn get_lca(&self, mut x: usize, mut y: usize) -> (usize, [i32; 26]) {
        if self.depth[x] > self.depth[y] {
            std::mem::swap(&mut x, &mut y);
        }
        let tmp = self.get_kth_ancestor(y as i32, self.depth[y] - self.depth[x]);
        y = tmp.0 as usize;
        let mut cw = tmp.1;
        if y != x {
            for i in (0..self.pa[x].len() - 1).rev() {
                if self.pa[x][i] != self.pa[y][i] {
                    for (j, item) in cw.iter_mut().enumerate() {
                        *item += self.cnt[x][i][j] + self.cnt[y][i][j];
                    }
                    x = self.pa[x][i] as usize;
                    y = self.pa[y][i] as usize;
                }
            }
            for (j, item) in cw.iter_mut().enumerate() {
                *item += self.cnt[x][0][j] + self.cnt[y][0][j];
            }
            x = self.pa[x][0] as usize;
        }
        (x, cw)
    }
}

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let tree = Tree::new(n as usize, edges);
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let x = query[0] as usize;
            let y = query[1] as usize;
            let (lca, cw) = tree.get_lca(x, y);
            let path_len = tree.depth[x] + tree.depth[y] - 2 * tree.depth[lca];
            result.push(path_len - IntoIterator::into_iter(cw).max().unwrap());
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
            Solution::min_operations_queries(
                7,
                [
                    [0, 1, 1],
                    [1, 2, 1],
                    [2, 3, 1],
                    [3, 4, 2],
                    [4, 5, 2],
                    [5, 6, 2]
                ]
                .to_vec_vec(),
                [[0, 3], [3, 6], [2, 6], [0, 6]].to_vec_vec()
            ),
            [0, 0, 1, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_operations_queries(
                8,
                [
                    [1, 2, 6],
                    [1, 3, 4],
                    [2, 4, 6],
                    [2, 5, 3],
                    [3, 6, 6],
                    [3, 0, 8],
                    [7, 0, 2]
                ]
                .to_vec_vec(),
                [[4, 6], [0, 4], [6, 5], [7, 4]].to_vec_vec()
            ),
            [1, 2, 2, 3]
        );
    }
}
