pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        Solution::foo2(nums1, nums2)
    }

    fn foo1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn get_k_num(nums1: &[i32], nums2: &[i32], k: usize) -> f64 {
            if nums1.is_empty() {
                return nums2[k - 1] as f64;
            }
            if nums2.is_empty() {
                return nums1[k - 1] as f64;
            }
            if k == 1 {
                return nums1[0].min(nums2[0]) as f64;
            }

            let index = (k / 2 - 1).min(nums1.len() - 1).min(nums2.len() - 1);
            let num1 = nums1[index];
            let num2 = nums2[index];
            if num1 <= num2 {
                get_k_num(&nums1[index + 1..], nums2, k - index - 1)
            } else {
                get_k_num(nums1, &nums2[index + 1..], k - index - 1)
            }
        }

        let total_size = nums1.len() + nums2.len();
        if total_size & 1 == 1 {
            get_k_num(&nums1, &nums2, (total_size + 1) / 2)
        } else {
            (get_k_num(&nums1, &nums2, total_size / 2)
                + get_k_num(&nums1, &nums2, total_size / 2 + 1))
                / 2.0
        }
    }

    fn foo2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let total_size = nums1.len() + nums2.len();
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let mut left = 0;
        let mut right = nums1.len();
        let mut left_max: i32 = 0;
        let mut right_min: i32 = 0;
        while left <= right {
            let i = (right - left) / 2 + left;
            let j = (total_size + 1) / 2 - i;

            let nums1_i_1 = if i == 0 { std::i32::MIN } else { nums1[i - 1] };
            let nums1_i = if i == nums1.len() {
                std::i32::MAX
            } else {
                nums1[i]
            };
            let nums2_j_1 = if j == 0 { std::i32::MIN } else { nums2[j - 1] };
            let nums2_j = if j == nums2.len() {
                std::i32::MAX
            } else {
                nums2[j]
            };

            if nums1_i_1 <= nums2_j {
                left_max = nums1_i_1.max(nums2_j_1);
                right_min = nums1_i.min(nums2_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if total_size & 1 == 1 {
            left_max as f64
        } else {
            (left_max + right_min) as f64 / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2f64
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
