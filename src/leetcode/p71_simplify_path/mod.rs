pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path = path.split('/');
        let mut stack = Vec::new();
        for p in path {
            match p {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                _ => stack.push(p),
            }
        }
        "/".to_string() + &stack.join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/");
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo"
        );
    }
}
