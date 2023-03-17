pub struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .into_iter()
            .zip(end_time.into_iter())
            .filter(|(start, end)| *start <= query_time && query_time <= *end)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::busy_student([1, 2, 3].to_vec(), [3, 2, 7].to_vec(), 4),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::busy_student([4].to_vec(), [4].to_vec(), 4), 1);
    }
}
