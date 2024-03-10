pub struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut count = [0; 10];
        for (s, g) in secret.bytes().zip(guess.bytes()) {
            if s == g {
                a += 1;
            } else {
                let s = (s - b'0') as usize;
                let g = (g - b'0') as usize;
                if count[g] > 0 {
                    b += 1;
                }
                count[g] -= 1;
                if count[s] < 0 {
                    b += 1;
                }
                count[s] += 1;
            }
        }
        format!("{}A{}B", a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B"
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B"
        );
    }
}
