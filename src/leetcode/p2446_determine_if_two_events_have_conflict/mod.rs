pub struct Solution;

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        event1[0] <= event2[1] && event1[1] >= event2[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert!(Solution::have_conflict(
            ["01:15", "02:00"].to_vec_string(),
            ["02:00", "03:00"].to_vec_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(Solution::have_conflict(
            ["01:00", "02:00"].to_vec_string(),
            ["01:20", "03:00"].to_vec_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::have_conflict(
            ["10:00", "11:00"].to_vec_string(),
            ["14:00", "15:00"].to_vec_string()
        ));
    }
}
