pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
        let mut i = 0 as usize;
        let num = strs.len() as i32;
        if num == 0 {
            "".to_string()
        } else if num == 1 {
            strs[0].iter().collect()
        } else {
            'a: loop {
                let mut j = 0 as usize;
                while (j as i32) < num - 1 {
                    if !(i < strs[j].len() && i < strs[j + 1].len() && strs[j][i] == strs[j + 1][i])
                    {
                        break 'a;
                    }
                    j += 1;
                }
                i += 1;
            }
            if i == 0 {
                "".to_string()
            } else {
                strs[0][0..i].iter().collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let test = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(Solution::longest_common_prefix(test), "fl");
    }

    #[test]
    fn test2() {
        let test = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(Solution::longest_common_prefix(test), "");
    }
}
