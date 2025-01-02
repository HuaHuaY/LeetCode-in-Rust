use std::collections::BTreeMap;

#[allow(dead_code)]
struct MyCalendar {
    map: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some((_, &before_e)) = self.map.range(..start_time).next_back() {
            if before_e > start_time {
                return false;
            }
        }
        if let Some((&after_s, _)) = self.map.range(start_time..).next() {
            if after_s < end_time {
                return false;
            }
        }
        self.map.insert(start_time, end_time);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(startTime, endTime);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut my_calendar = MyCalendar::new();
        assert!(my_calendar.book(10, 20));
        assert!(!my_calendar.book(15, 25));
        assert!(my_calendar.book(20, 30));
    }
}
