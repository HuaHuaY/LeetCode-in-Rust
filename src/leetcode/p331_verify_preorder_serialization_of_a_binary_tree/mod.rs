pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let nodes = preorder.split(',');
        let mut degree_diff = 1;

        for i in nodes {
            degree_diff -= 1;
            if degree_diff < 0 {
                return false;
            }
            if i != "#" {
                degree_diff += 2;
            }
        }
        degree_diff == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_valid_serialization(String::from(
            "9,3,4,#,#,1,#,#,2,#,6,#,#"
        )));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_valid_serialization(String::from("1,#")));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_valid_serialization(String::from("9,#,#,1")));
    }
}
