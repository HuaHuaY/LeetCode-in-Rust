pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        Self::foo1(num)
    }

    fn foo1(num: i32) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::with_capacity(num as usize + 1);
        unsafe {
            results.set_len(num as usize + 1);
        }
        results[0] = 0;
        for i in 1..results.len() {
            results[i] = results[i >> 1] + (i as i32 & 1);
        }
        results
    }

    fn foo2(num: i32) -> Vec<i32> {
        (0..=num).map(|x| x.count_ones() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_bits(2), [0, 1, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_bits(5), [0, 1, 1, 2, 1, 2]);
    }
}
