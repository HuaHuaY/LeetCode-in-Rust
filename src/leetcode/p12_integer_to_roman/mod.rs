pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const MAP: [(i32, &str); 13] = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        for (k, v) in MAP {
            while num >= k {
                result.push_str(v);
                num -= k;
            }
            if num == 0 {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::int_to_roman(3), "III");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
