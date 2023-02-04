pub struct Solution;

use std::collections::HashMap;

struct UnionFind {
    root: Vec<Vec<(usize, usize)>>,
    size: Vec<Vec<usize>>,
}

impl UnionFind {
    fn new(m: usize, n: usize) -> Self {
        Self {
            root: (0..m).map(|i| (0..n).map(|j| (i, j)).collect()).collect(),
            size: vec![vec![1; n]; m],
        }
    }

    fn find(&mut self, i: usize, j: usize) -> (usize, usize) {
        let mut root = self.root[i][j];
        if root == (i, j) {
            root
        } else {
            root = self.find(root.0, root.1);
            self.root[i][j] = root;
            root
        }
    }

    fn union(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        let root1 = self.find(i1, j1);
        let root2 = self.find(i2, j2);
        if root1 == root2 {
            return;
        }

        if self.size[root1.0][root1.1] < self.size[root2.0][root2.1] {
            self.root[root1.0][root1.1] = root2;
            self.size[root2.0][root2.1] += self.size[root1.0][root1.1];
        } else {
            self.root[root2.0][root2.1] = root1;
            self.size[root1.0][root1.1] += self.size[root2.0][root2.1];
        }
    }
}

impl Solution {
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut uf = UnionFind::new(matrix.len(), matrix[0].len());

        let mut line_sort_matrix = Vec::with_capacity(matrix.len());
        for (i, line) in matrix.iter().enumerate() {
            let mut indices = (0..line.len()).collect::<Vec<_>>();
            indices.sort_unstable_by_key(|index| line[*index]);
            let mut prev_j = indices[0];
            for &j in indices.iter().skip(1) {
                if line[j] == line[prev_j] {
                    uf.union(i, j, i, prev_j);
                } else {
                    prev_j = j;
                }
            }
            line_sort_matrix.push(indices);
        }

        let mut column_sort_matrix = Vec::with_capacity(matrix[0].len());
        for j in 0..matrix[0].len() {
            let mut indices = (0..matrix.len()).collect::<Vec<_>>();
            indices.sort_unstable_by_key(|index| matrix[*index][j]);
            let mut prev_i = indices[0];
            for &i in indices.iter().skip(1) {
                if matrix[i][j] == matrix[prev_i][j] {
                    uf.union(i, j, prev_i, j);
                } else {
                    prev_i = i;
                }
            }
            column_sort_matrix.push(indices);
        }

        let mut degree = HashMap::new();
        let mut next = HashMap::new();
        for (i, line) in line_sort_matrix.into_iter().enumerate() {
            let mut prev_root = uf.find(i, line[0]);
            for j in line.into_iter().skip(1) {
                let root = uf.find(i, j);
                if prev_root != root {
                    degree.entry(root).and_modify(|e| *e += 1).or_insert(1);
                    next.entry(prev_root).or_insert_with(Vec::new).push(root);
                    prev_root = root;
                }
            }
        }

        for (j, column) in column_sort_matrix.into_iter().enumerate() {
            let mut prev_root = uf.find(column[0], j);
            for i in column.into_iter().skip(1) {
                let root = uf.find(i, j);
                if prev_root != root {
                    degree.entry(root).and_modify(|e| *e += 1).or_insert(1);
                    next.entry(prev_root).or_insert_with(Vec::new).push(root);
                    prev_root = root;
                }
            }
        }

        let mut rank = 1;
        let mut rank_vec1 = Vec::new();
        let mut rank_vec2 = Vec::new();
        let mut result = vec![vec![1; matrix[0].len()]; matrix.len()];
        for prev_root in next.keys() {
            if !degree.contains_key(prev_root) {
                rank_vec1.push(*prev_root);
            }
        }
        while !rank_vec1.is_empty() {
            for &(i, j) in &rank_vec1 {
                result[i][j] = rank;
                if let Some(next) = next.remove(&(i, j)) {
                    for root in next {
                        let v = degree.get_mut(&root).unwrap();
                        *v -= 1;
                        if *v == 0 {
                            rank_vec2.push(root);
                        }
                    }
                }
            }
            rank += 1;
            std::mem::swap(&mut rank_vec1, &mut rank_vec2);
            rank_vec2.clear();
        }

        for i in 0..result.len() {
            for j in 0..result[0].len() {
                if result[i][j] == 1 {
                    let (ri, rj) = uf.find(i, j);
                    result[i][j] = result[ri][rj];
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::common::ToVecVec;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::matrix_rank_transform([[1, 2], [3, 4]].to_vec_vec()),
            [[1, 2], [2, 3]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::matrix_rank_transform([[7, 7], [7, 7]].to_vec_vec()),
            [[1, 1], [1, 1]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::matrix_rank_transform(
                [[20, -21, 14], [-19, 4, 19], [22, -47, 24], [-19, 4, 19]].to_vec_vec()
            ),
            [[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::matrix_rank_transform(
                [
                    [-37, -50, -3, 44],
                    [-37, 46, 13, -32],
                    [47, -42, -3, -40],
                    [-17, -22, -39, 24]
                ]
                .to_vec_vec()
            ),
            [[2, 1, 4, 6], [2, 6, 5, 4], [5, 2, 4, 3], [4, 3, 1, 5]]
        );
    }
}
