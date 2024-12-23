use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

#[allow(dead_code)]
#[derive(Default)]
struct ExamRoom {
    n: i32,
    heap: BinaryHeap<(i32, Reverse<i32>, i32)>,
    set: BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            ..Default::default()
        }
    }

    fn seat(&mut self) -> i32 {
        if self.set.is_empty() {
            self.set.insert(0);
            return 0;
        }

        let left = *self.set.first().unwrap();
        let right = *self.set.last().unwrap();

        if self.set.len() >= 2 {
            while let Some(&(d, Reverse(l), r)) = self.heap.peek() {
                if self.set.iter().any(|&i| l <= i && i <= r) {
                    self.heap.pop();
                    continue;
                }

                if d <= left || d < self.n - 1 - right {
                    break;
                }

                self.heap.pop();
                let mid = (r + l) / 2;
                if mid != l {
                    self.heap.push(((mid - 1 - l) / 2 + 1, Reverse(l), mid - 1));
                }
                if mid != r {
                    self.heap.push(((r - mid - 1) / 2 + 1, Reverse(mid + 1), r));
                }
                self.set.insert(mid);
                return mid;
            }
        }

        if left < self.n - 1 - right {
            if right + 1 != self.n - 1 {
                self.heap
                    .push(((self.n - right - 3) / 2 + 1, Reverse(right + 1), self.n - 2));
            }
            self.set.insert(self.n - 1);
            self.n - 1
        } else {
            if left - 1 != 0 {
                self.heap.push(((left - 2) / 2 + 1, Reverse(1), left - 1));
            }
            self.set.insert(0);
            0
        }
    }

    fn leave(&mut self, p: i32) {
        let left = *self.set.first().unwrap();
        let right = *self.set.last().unwrap();
        if p != left && p != right {
            let prev = *self.set.range(..p).next_back().unwrap();
            let next = *self.set.range(p + 1..).next().unwrap();
            self.heap
                .push(((next - prev - 2) / 2 + 1, Reverse(prev + 1), next - 1));
        }
        self.set.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut exam_room = ExamRoom::new(10);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 9);
        assert_eq!(exam_room.seat(), 4);
        assert_eq!(exam_room.seat(), 2);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 5);
    }

    #[test]
    fn test2() {
        let mut exam_room = ExamRoom::new(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 3);
        assert_eq!(exam_room.seat(), 1);
        assert_eq!(exam_room.seat(), 2);
        exam_room.leave(1);
        exam_room.leave(3);
        assert_eq!(exam_room.seat(), 1);
    }

    #[test]
    fn test3() {
        let mut exam_room = ExamRoom::new(10);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 9);
        assert_eq!(exam_room.seat(), 4);
        exam_room.leave(0);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 4);
        assert_eq!(exam_room.seat(), 2);
        assert_eq!(exam_room.seat(), 6);
        assert_eq!(exam_room.seat(), 1);
        assert_eq!(exam_room.seat(), 3);
        assert_eq!(exam_room.seat(), 5);
        assert_eq!(exam_room.seat(), 7);
        assert_eq!(exam_room.seat(), 8);
        exam_room.leave(0);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 4);
    }
}
