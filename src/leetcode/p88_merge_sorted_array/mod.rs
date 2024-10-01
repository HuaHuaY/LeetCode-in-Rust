pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut [i32], n: i32) {
        let mut p = nums1.len() as i32 - 1;
        let mut i = m - 1;
        let mut j = n - 1;
        while i >= 0 && j >= 0 {
            nums1[p as usize] = if nums1[i as usize] > nums2[j as usize] {
                i -= 1;
                nums1[(i + 1) as usize]
            } else {
                j -= 1;
                nums2[(j + 1) as usize]
            };
            p -= 1;
        }
        while j >= 0 {
            nums1[p as usize] = nums2[j as usize];
            j -= 1;
            p -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums1 = [1, 2, 3, 0, 0, 0].to_vec();
        let mut nums2 = [2, 5, 6].to_vec();
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test2() {
        let mut nums1 = [1].to_vec();
        let mut nums2 = [].to_vec();
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn test3() {
        let mut nums1 = [0].to_vec();
        let mut nums2 = [1].to_vec();
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, [1]);
    }
}
