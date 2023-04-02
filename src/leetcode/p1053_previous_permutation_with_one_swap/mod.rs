pub struct Solution;

impl Solution {
    pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
        let mut i = arr.len() - 1;
        while i > 0 && arr[i - 1] <= arr[i] {
            i -= 1;
        }
        if i != 0 {
            let k = i - 1;
            match arr[i..].binary_search(&(arr[k] - 1)) {
                Ok(idx) => {
                    let mut idx = i + idx;
                    while idx > 0 && arr[idx - 1] == arr[idx] {
                        idx -= 1;
                    }
                    arr[idx] = arr[k];
                    arr[k] -= 1;
                }
                Err(idx) => {
                    let mut idx = i + idx - 1;
                    while idx > 0 && arr[idx - 1] == arr[idx] {
                        idx -= 1;
                    }
                    arr.swap(k, idx);
                }
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::prev_perm_opt1([3, 2, 1].to_vec()), [3, 1, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::prev_perm_opt1([1, 1, 5].to_vec()), [1, 1, 5]);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::prev_perm_opt1([1, 9, 4, 6, 7].to_vec()),
            [1, 7, 4, 6, 9]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::prev_perm_opt1([3, 1, 1, 3].to_vec()),
            [1, 3, 1, 3]
        );
    }
}
