pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = version1.split('.').collect::<Vec<_>>();
        let version2 = version2.split('.').collect::<Vec<_>>();
        let min_len = version1.len().min(version2.len());
        for (v1, v2) in version1.iter().zip(version2.iter()) {
            let v1 = v1.parse::<u32>().unwrap();
            let v2 = v2.parse::<u32>().unwrap();
            match v1.cmp(&v2) {
                Ordering::Greater => return 1,
                Ordering::Less => return -1,
                Ordering::Equal => continue,
            }
        }
        for v1 in version1.into_iter().skip(min_len) {
            let v1 = v1.parse::<u32>().unwrap();
            if v1 != 0 {
                return 1;
            }
        }
        for v2 in version2.into_iter().skip(min_len) {
            let v2 = v2.parse::<u32>().unwrap();
            if v2 != 0 {
                return -1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string()),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
            0
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::compare_version("0.1".to_string(), "1.1".to_string()),
            -1
        );
    }
}
