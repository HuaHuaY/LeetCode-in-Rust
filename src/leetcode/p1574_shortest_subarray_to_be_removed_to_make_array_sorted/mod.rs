pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = arr.len() - 1;
        while j > 0 && arr[j - 1] <= arr[j] {
            j -= 1;
        }
        if j == 0 {
            return 0;
        }
        let mut min = j;
        let t = j;
        while i < t {
            while j < arr.len() && arr[i] > arr[j] {
                j += 1;
            }
            min = min.min(j - i - 1);
            if arr[i] <= arr[i + 1] {
                i += 1;
            } else {
                break;
            }
        }
        min as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray([1, 2, 3, 10, 4, 2, 3, 5].to_vec()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray([5, 4, 3, 2, 1].to_vec()),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_length_of_shortest_subarray([1, 2, 3].to_vec()),
            0
        );
    }
}
