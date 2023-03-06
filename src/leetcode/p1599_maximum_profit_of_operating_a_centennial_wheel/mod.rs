pub struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        if boarding_cost * 4 <= running_cost {
            return -1;
        }

        let len = customers.len();
        let mut wait_people = 0;
        let mut profit = 0;
        let mut max_profit = 0;
        let mut min_operations = 0;
        for (i, customer) in customers.into_iter().enumerate() {
            wait_people += customer;
            profit += boarding_cost * wait_people.min(4) - running_cost;
            wait_people -= wait_people.min(4);
            if profit > max_profit {
                max_profit = profit;
                min_operations = i + 1;
            }
        }
        let mut operation = 0;
        while wait_people > 0 {
            profit += boarding_cost * wait_people.min(4) - running_cost;
            wait_people -= wait_people.min(4);
            operation += 1;
            if profit > max_profit {
                max_profit = profit;
                min_operations = len + operation;
            }
        }
        if max_profit == 0 {
            -1
        } else {
            min_operations as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_operations_max_profit([8, 3].to_vec(), 5, 6),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_operations_max_profit([10, 9, 6].to_vec(), 6, 4),
            7
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::min_operations_max_profit([3, 4, 0, 5, 1].to_vec(), 1, 92),
            -1
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::min_operations_max_profit([5, 0, 0, 0, 0, 30].to_vec(), 5, 5),
            13
        );
    }
}
