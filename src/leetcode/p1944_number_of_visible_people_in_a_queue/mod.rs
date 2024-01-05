pub struct Solution;

impl Solution {
    pub fn can_see_persons_count(mut heights: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        let mut num;
        for item in heights.iter_mut().rev() {
            num = *item;
            *item = 0;
            while let Some(i) = stack.last() {
                if num > *i {
                    *item += 1;
                    stack.pop();
                } else {
                    break;
                }
            }
            if !stack.is_empty() {
                *item += 1;
            }
            stack.push(num);
        }
        heights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_see_persons_count([10, 6, 8, 5, 11, 9].to_vec()),
            [3, 1, 2, 1, 1, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_see_persons_count([5, 1, 2, 3, 10].to_vec()),
            [4, 1, 1, 1, 0]
        );
    }
}
