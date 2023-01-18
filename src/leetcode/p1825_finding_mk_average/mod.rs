use std::collections::{BTreeMap, VecDeque};

#[allow(dead_code)]
struct MKAverage {
    m: usize,
    k: usize,
    queue: VecDeque<i32>,
    tree: BTreeMap<i32, usize>,
    sum: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            m: m as usize,
            k: k as usize,
            queue: VecDeque::new(),
            tree: BTreeMap::new(),
            sum: 0,
        }
    }

    fn add_element(&mut self, num: i32) {
        self.queue.push_back(num);
        self.sum += num;
        self.tree.entry(num).and_modify(|e| *e += 1).or_insert(1);
        if self.queue.len() <= self.m {
            return;
        }

        let remove = self.queue.pop_front().unwrap();
        self.sum -= remove;
        let v = self.tree.get_mut(&remove).unwrap();
        *v -= 1;
        if *v == 0 {
            self.tree.remove(&remove);
        }
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.queue.len() < self.m {
            return -1;
        }

        let min_sum = self
            .tree
            .iter()
            .flat_map(|(k, v)| std::iter::repeat(k).take(*v))
            .take(self.k)
            .sum::<i32>();
        let max_sum = self
            .tree
            .iter()
            .rev()
            .flat_map(|(k, v)| std::iter::repeat(k).take(*v))
            .take(self.k)
            .sum::<i32>();

        (self.sum - min_sum - max_sum) / (self.m - 2 * self.k) as i32
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MKAverage::new(3, 1);
        obj.add_element(3);
        obj.add_element(1);
        assert_eq!(obj.calculate_mk_average(), -1);
        obj.add_element(10);
        assert_eq!(obj.calculate_mk_average(), 3);
        obj.add_element(5);
        obj.add_element(5);
        obj.add_element(5);
        assert_eq!(obj.calculate_mk_average(), 5);
    }
}
