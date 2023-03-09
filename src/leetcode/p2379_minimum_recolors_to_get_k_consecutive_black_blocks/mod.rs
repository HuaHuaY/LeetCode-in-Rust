pub struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.bytes().collect::<Vec<_>>();
        let mut count = 0;
        let mut min = i32::MAX;
        for i in 0..blocks.len() {
            if blocks[i] == b'W' {
                count += 1;
            }
            if i >= k as usize && blocks[i - k as usize] == b'W' {
                count -= 1;
            }
            if i >= k as usize - 1 {
                min = min.min(count);
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }
}
