pub struct Solution;

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut result = String::new();
        let mut x = 0;
        let mut y = 0;
        for c in target.bytes().map(|c| c - b'a') {
            let x1 = c as usize % 5;
            let y1 = c as usize / 5;
            if y1 < y {
                result.push_str(&"U".repeat(y - y1));
            }
            if x1 < x {
                result.push_str(&"L".repeat(x - x1));
            }
            if y1 > y {
                result.push_str(&"D".repeat(y1 - y));
            }
            if x1 > x {
                result.push_str(&"R".repeat(x1 - x));
            }
            result.push('!');
            x = x1;
            y = y1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::alphabet_board_path("leet".to_string()),
            "DDR!UURRR!!DDD!"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::alphabet_board_path("code".to_string()),
            "RR!DDRR!UUL!R!"
        );
    }
}
