pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut answer = vec![0; num1.len() + num2.len()];
        let num1 = num1.bytes().map(|b| (b - b'0') as u32).collect::<Vec<_>>();
        let num2 = num2.bytes().map(|b| (b - b'0') as u32).collect::<Vec<_>>();
        for (i, n1) in num1.iter().rev().enumerate() {
            for (j, n2) in num2.iter().rev().enumerate() {
                let index = i + j;
                let product = (*n1) * (*n2);
                answer[index] += product;
            }
        }
        for i in 0..answer.len() {
            if answer[i] >= 10 {
                answer[i + 1] += answer[i] / 10;
                answer[i] %= 10;
            }
        }
        answer
            .into_iter()
            .rev()
            .skip_while(|i| *i == 0)
            .map(|i| i.to_string())
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088"
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::multiply("999".to_string(), "999".to_string()),
            "998001"
        );
    }
}
