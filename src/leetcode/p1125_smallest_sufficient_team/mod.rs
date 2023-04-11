pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let skills = req_skills
            .into_iter()
            .enumerate()
            .map(|(i, e)| (e, i))
            .collect::<HashMap<_, _>>();
        let m = people.len();
        let mut dp = vec![m; 1 << skills.len()];
        let mut pre_skill = vec![0; 1 << skills.len()];
        let mut pre_people = vec![0; 1 << skills.len()];
        dp[0] = 0;
        for (i, people) in people.into_iter().enumerate() {
            let mut cur = 0;
            for skill in people {
                cur |= 1 << *skills.get(&skill).unwrap();
            }
            for j in 0..dp.len() {
                if dp[j] == m {
                    continue;
                }
                let comb = j | cur;
                if comb == j {
                    continue;
                }
                if dp[comb] > dp[j] + 1 {
                    dp[comb] = dp[j] + 1;
                    pre_people[comb] = i;
                    pre_skill[comb] = j;
                }
            }
        }
        let mut result = vec![];
        let mut i = (1 << skills.len()) - 1;
        while i > 0 {
            result.push(pre_people[i] as i32);
            i = pre_skill[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ToVecString;
    use crate::vec_vec_string;

    #[test]
    fn test1() {
        let mut vec = Solution::smallest_sufficient_team(
            ["java", "nodejs", "reactjs"].to_vec_string(),
            vec_vec_string![["java"], ["nodejs"], ["nodejs", "reactjs"]],
        );
        vec.sort_unstable();
        let mut answer = [0, 2];
        answer.sort_unstable();
        assert_eq!(vec, answer);
    }

    #[test]
    fn test2() {
        let mut vec = Solution::smallest_sufficient_team(
            ["algorithms", "math", "java", "reactjs", "csharp", "aws"].to_vec_string(),
            vec_vec_string![
                ["algorithms", "math", "java"],
                ["algorithms", "math", "reactjs"],
                ["java", "csharp", "aws"],
                ["reactjs", "csharp"],
                ["csharp", "math"],
                ["aws", "java"]
            ],
        );
        vec.sort_unstable();
        let mut answer = [1, 2];
        answer.sort_unstable();
        assert_eq!(vec, answer);
    }
}
