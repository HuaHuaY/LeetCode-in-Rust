static DOLLARS: [i32; 5] = [20, 50, 100, 200, 500];

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
struct ATM {
    count: [i32; 5],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl ATM {
    fn new() -> Self {
        Self { count: [0; 5] }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        self.count
            .iter_mut()
            .zip(banknotes_count)
            .for_each(|(a, b)| *a += b);
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let result = DOLLARS
            .into_iter()
            .enumerate()
            .rev()
            .fold(vec![0; 5], |mut acc, (idx, i)| {
                let c = self.count[idx].min(amount / i);
                acc[idx] += c;
                self.count[idx] -= c;
                amount -= c * i;
                acc
            });
        if amount == 0 {
            result
        } else {
            self.deposit(result);
            vec![-1]
        }
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut atm = ATM::new();
        atm.deposit([0, 0, 1, 2, 1].to_vec());
        assert_eq!(atm.withdraw(600), [0, 0, 1, 0, 1]);
        atm.deposit([0, 1, 0, 1, 1].to_vec());
        assert_eq!(atm.withdraw(600), [-1]);
        assert_eq!(atm.withdraw(550), [0, 1, 0, 0, 1]);
    }
}
