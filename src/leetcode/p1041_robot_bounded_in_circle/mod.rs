pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut now = (0, 0);
        let mut direct = 0;
        const DIRECT: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for c in instructions.chars() {
            match c {
                'G' => now = (now.0 + DIRECT[direct].0, now.1 + DIRECT[direct].1),
                'L' => direct = (direct + 3) % 4,
                'R' => direct = (direct + 1) % 4,
                _ => unreachable!(),
            }
        }
        now == (0, 0) || direct != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_robot_bounded("GGLLGG".to_string()));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_robot_bounded("GG".to_string()));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_robot_bounded("GL".to_string()));
    }
}
