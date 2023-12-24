pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut cache = vec![-1; n as usize];
        let mut stack = vec![];
        cache[head_id as usize] = 0;
        for i in 0..n as usize {
            if cache[i] != -1 {
                continue;
            }
            stack.push(i);
            let mut mentor = manager[i] as usize;
            while cache[mentor] == -1 {
                stack.push(mentor);
                mentor = manager[mentor] as usize;
            }
            while let Some(top) = stack.pop() {
                cache[top] = cache[mentor] + inform_time[mentor];
                mentor = top;
            }
        }
        cache.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_of_minutes(1, 0, [-1].to_vec(), [0].to_vec()),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_of_minutes(
                6,
                2,
                [2, 2, -1, 2, 2, 2].to_vec(),
                [0, 0, 1, 0, 0, 0].to_vec()
            ),
            1
        );
    }
}
