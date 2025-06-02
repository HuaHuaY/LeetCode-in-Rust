use std::collections::BTreeMap;

#[allow(dead_code)]
struct MyCalendarThree {
    map: BTreeMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn update(&mut self, start: i32, end: i32, value: i32, left: i32, right: i32, idx: i32) {
        if end < left || start > right {
            return;
        }
        self.map.entry(idx).or_default();
        if start <= left && right <= end {
            self.map.entry(idx).and_modify(|(a, b)| {
                *a += value;
                *b += value;
            });
        } else {
            let mid = (right - left) / 2 + left;
            self.update(start, end, value, left, mid, idx * 2);
            self.update(start, end, value, mid + 1, right, idx * 2 + 1);
            let l = self.map.entry(idx * 2).or_default().0;
            let r = self.map.entry(idx * 2 + 1).or_default().0;
            self.map.entry(idx).and_modify(|(a, b)| *a = *b + l.max(r));
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.update(start_time, end_time - 1, 1, 0, 1_000_000_000, 1);
        self.map.get(&1).unwrap().0
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(startTime, endTime);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_calendar = MyCalendarThree::new();
        assert_eq!(my_calendar.book(10, 20), 1);
        assert_eq!(my_calendar.book(50, 60), 1);
        assert_eq!(my_calendar.book(10, 40), 2);
        assert_eq!(my_calendar.book(5, 15), 3);
        assert_eq!(my_calendar.book(5, 10), 3);
        assert_eq!(my_calendar.book(25, 55), 3);
    }
}
