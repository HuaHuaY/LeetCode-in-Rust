pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut tmp = (1..=k).collect::<Vec<_>>();
        let mut i = 0;
        while i < k as usize && tmp[i] != n + 1 {
            result.push(tmp.clone());
            i = 0;
            while i + 1 < k as usize && tmp[i] + 1 == tmp[i + 1] {
                tmp[i] = i as i32 + 1;
                i += 1;
            }
            tmp[i] += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut vec = Solution::combine(4, 2);
        vec.sort_unstable();
        let mut ans = [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]].to_vec();
        ans.sort_unstable();
        assert_eq!(vec, ans);
    }

    #[test]
    fn test2() {
        let mut vec = Solution::combine(1, 1);
        vec.sort_unstable();
        let mut ans = [[1]].to_vec();
        ans.sort_unstable();
        assert_eq!(vec, ans);
    }
}
