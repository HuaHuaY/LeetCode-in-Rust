pub struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut pos = vec![vec![]; 26];
        for (i, b) in ring.bytes().enumerate() {
            pos[(b - b'a') as usize].push(i);
        }

        let len = ring.len();
        let mut dp1 = vec![0; len];
        let mut dp2 = dp1.clone();

        for p in &pos[(key.as_bytes()[0] - b'a') as usize] {
            dp1[*p] = (*p).min(len - *p) + 1;
        }
        for (k_pre, k) in key
            .bytes()
            .map(|b| b - b'a')
            .zip(key.bytes().skip(1).map(|b| b - b'a'))
        {
            for p in &pos[k as usize] {
                dp2[*p] = usize::MAX;
                for p_pre in &pos[k_pre as usize] {
                    let abs = (*p).abs_diff(*p_pre);
                    dp2[*p] = dp2[*p].min(dp1[*p_pre] + abs.min(len - abs) + 1);
                }
            }
            std::mem::swap(&mut dp1, &mut dp2);
        }

        dp1.into_iter()
            .enumerate()
            .filter(|(k, _)| *key.as_bytes().last().unwrap() == ring.as_bytes()[*k])
            .map(|(_, v)| v)
            .min()
            .unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "gd".to_string()),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_string(), "godding".to_string()),
            13
        );
    }
}
