pub struct Solution;

pub struct TreeAncestor {
    pa: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    pub fn new(n: i32, parent: Vec<i32>) -> Self {
        let m = n.ilog2() as usize + 1;
        let mut pa = vec![vec![-1; m]; n as usize];
        for (i, v) in parent.into_iter().enumerate() {
            pa[i][0] = v;
        }
        for j in 0..m - 1 {
            for i in 0..n as usize {
                if pa[i][j] != -1 {
                    pa[i][j + 1] = pa[pa[i][j] as usize][j];
                }
            }
        }
        TreeAncestor { pa }
    }

    pub fn get_kth_ancestor(&self, mut node: i32, mut k: i32) -> i32 {
        while k != 0 && node != -1 {
            node = self.pa[node as usize][k.trailing_zeros() as usize];
            k &= k - 1;
        }
        node
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tree_ancestor = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(tree_ancestor.get_kth_ancestor(3, 1), 1);
        assert_eq!(tree_ancestor.get_kth_ancestor(5, 2), 0);
        assert_eq!(tree_ancestor.get_kth_ancestor(6, 3), -1);
    }
}
