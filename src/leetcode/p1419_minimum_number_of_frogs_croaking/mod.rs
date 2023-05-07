pub struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut slots = vec![0; 4];
        let mut count = 0;
        let mut max = 0;
        for c in croak_of_frogs.chars() {
            let index = match c {
                'c' => 0,
                'r' => 1,
                'o' => 2,
                'a' => 3,
                'k' => 4,
                _ => unreachable!(),
            };
            if index > 0 && slots[index - 1] == 0 {
                return -1;
            }
            match index {
                0 => {
                    count += 1;
                    slots[0] += 1;
                    max = max.max(count)
                }
                4 => {
                    count -= 1;
                    slots[3] -= 1;
                }
                _ => {
                    slots[index - 1] -= 1;
                    slots[index] += 1;
                }
            }
        }
        if count != 0 {
            return -1;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
    }
}
