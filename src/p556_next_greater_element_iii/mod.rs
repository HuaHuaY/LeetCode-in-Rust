pub struct Solution {}

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut str = n.to_string().chars().collect::<Vec<char>>();
        let length = str.len();

        let mut i = length as i32 - 2;
        while i >= 0 && str[i as usize] >= str[i as usize + 1] {
            i -= 1;
        }

        if i == -1 {
            -1
        } else {
            let mut j = length - 1;
            while str[i as usize] >= str[j] {
                j -= 1;
            }

            let tmp = str[i as usize];
            str[i as usize] = str[j];
            str[j] = tmp;

            str[i as usize + 1..].reverse();

            str.into_iter().collect::<String>().parse().unwrap_or(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::next_greater_element(12), 21);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::next_greater_element(21), -1);
    }
}
