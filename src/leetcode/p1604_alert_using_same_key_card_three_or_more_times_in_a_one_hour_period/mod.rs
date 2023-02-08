pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for (name, time) in key_name.into_iter().zip(key_time.into_iter()) {
            let time = time
                .split(':')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let time = time[0] * 60 + time[1];
            map.entry(name).or_insert_with(Vec::new).push(time);
        }
        let mut names = map
            .into_iter()
            .filter_map(|(name, mut times)| {
                times.sort_unstable();
                let mut i = 0;
                while i + 2 < times.len() {
                    if times[i + 2] - times[i] <= 60 {
                        return Some(name);
                    }
                    i += 1;
                }
                None
            })
            .collect::<Vec<_>>();
        names.sort();
        names
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::alert_names(
                ["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"].to_vec_string(),
                ["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"].to_vec_string()
            ),
            ["daniel"]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::alert_names(
                ["alice", "alice", "alice", "bob", "bob", "bob", "bob"].to_vec_string(),
                ["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"].to_vec_string()
            ),
            ["bob"]
        );
    }
}
