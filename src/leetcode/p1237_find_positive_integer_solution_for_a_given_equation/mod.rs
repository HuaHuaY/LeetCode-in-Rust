pub struct Solution;

// This is the custom function interface.
// You should not implement it, or speculate about its implementation
pub struct CustomFunction;
impl CustomFunction {
    pub fn f(&self, _x: i32, _y: i32) -> i32 {
        todo!()
    }
}

use std::cmp::Ordering;

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut x = 1;
        let mut y = 1000;
        let mut result = vec![];
        while x <= 1000 && y >= 1 {
            let f = customfunction.f(x, y);
            match f.cmp(&z) {
                Ordering::Equal => {
                    result.push(vec![x, y]);
                    x += 1;
                    y -= 1;
                }
                Ordering::Greater => y -= 1,
                Ordering::Less => x += 1,
            }
        }
        result
    }
}
