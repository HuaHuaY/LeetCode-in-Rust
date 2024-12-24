pub struct Solution;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let len = apples.len();
        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        apples
            .into_iter()
            .zip(days)
            .enumerate()
            .fold(0, |result, (idx, (a, day))| {
                let idx = idx as i32;
                if a != 0 {
                    heap.push((Reverse(idx + day - 1), a));
                }
                if let Some(&(Reverse(max), apple)) = heap.peek() {
                    match max.cmp(&idx) {
                        Ordering::Less => unreachable!(),
                        Ordering::Equal => {
                            while let Some(&(Reverse(d), _)) = heap.peek() {
                                if max != d {
                                    break;
                                }
                                heap.pop();
                            }
                        }
                        Ordering::Greater if apple != 1 => {
                            heap.peek_mut().unwrap().1 = apple - 1;
                        }
                        Ordering::Greater => {
                            heap.pop();
                        }
                    }
                    result + 1
                } else {
                    result
                }
            })
            + heap
                .into_sorted_vec()
                .into_iter()
                .rev()
                .fold(
                    (0, len as i32 - 1),
                    |(result, max), (Reverse(day), apple)| {
                        if day > max {
                            (result + apple.min(day - max), max + apple.min(day - max))
                        } else {
                            (result, max)
                        }
                    },
                )
                .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::eaten_apples([1, 2, 3, 5, 2].to_vec(), [3, 2, 1, 4, 2].to_vec()),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::eaten_apples([3, 0, 0, 0, 0, 2].to_vec(), [3, 0, 0, 0, 0, 2].to_vec()),
            5
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::eaten_apples([2, 1, 10].to_vec(), [2, 10, 1].to_vec()),
            4
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::eaten_apples(
                [
                    1, 10, 17, 10, 12, 6, 20, 8, 8, 22, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 5, 2, 1, 0, 0, 0, 0, 0, 0, 23
                ]
                .to_vec(),
                [
                    19999, 11, 18, 22, 5, 2, 14, 5, 20, 7, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 1, 4, 2, 7, 0, 0, 0, 0, 0, 0, 1
                ]
                .to_vec()
            ),
            37
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::eaten_apples([2, 1, 1, 4, 5].to_vec(), [10, 10, 6, 4, 2].to_vec()),
            8
        );
    }
}
