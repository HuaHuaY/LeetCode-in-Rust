pub struct Solution;

impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut count = [0; 2];
        s1.bytes()
            .zip(s2.bytes())
            .filter(|(c1, c2)| *c1 != *c2)
            .for_each(|(c1, _)| count[(c1 - b'x') as usize] += 1);
        if (count[0] + count[1]) & 1 == 1 {
            -1
        } else {
            count[0] / 2 + count[1] / 2 + (count[0] & 1) * 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_swap("xx".to_string(), "yy".to_string()),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_swap("xy".to_string(), "yx".to_string()),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::minimum_swap("xx".to_string(), "xy".to_string()),
            -1
        );
    }
}
