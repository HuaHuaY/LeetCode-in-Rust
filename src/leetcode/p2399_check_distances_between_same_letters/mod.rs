pub struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut vec = vec![-1; 26];
        s.as_bytes()
            .iter()
            .map(|e| (e - b'a') as usize)
            .enumerate()
            .for_each(|(idx, b)| match vec[b] {
                -1 => vec[b] = idx as i32,
                _ => vec[b] = idx as i32 - vec[b] - 1,
            });
        vec.into_iter()
            .zip(distance)
            .filter(|(a, _)| *a != -1)
            .all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::check_distances(
            "abaccb".to_string(),
            [1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::check_distances(
            "aa".to_string(),
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ));
    }
}
