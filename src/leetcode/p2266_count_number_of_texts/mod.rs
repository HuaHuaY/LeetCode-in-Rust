pub struct Solution;

const MOD: i64 = 1_000_000_007;
static DP3: [i64; 100001] = {
    let mut arr: [i64; 100001] = [0; 100001];
    arr[0] = 1;
    arr[1] = 1;
    arr[2] = 2;
    let mut i = 3;
    while i <= 100000 {
        arr[i] = (arr[i - 1] + arr[i - 2] + arr[i - 3]) % MOD;
        i += 1;
    }
    arr
};
static DP4: [i64; 100001] = {
    let mut arr: [i64; 100001] = [0; 100001];
    arr[0] = 1;
    arr[1] = 1;
    arr[2] = 2;
    arr[3] = 4;
    let mut i = 4;
    while i <= 100000 {
        arr[i] = (arr[i - 1] + arr[i - 2] + arr[i - 3] + arr[i - 4]) % MOD;
        i += 1;
    }
    arr
};

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let mut result = 1i64;
        let mut count = 0;
        let mut pre_byte = b'\0';
        for b in pressed_keys.bytes() {
            if b == pre_byte {
                count += 1;
            } else {
                let dp = match pre_byte {
                    b'7' | b'9' => &DP4,
                    _ => &DP3,
                };
                result = (result * dp[count]) % MOD;
                pre_byte = b;
                count = 1;
            }
        }
        let dp = match pre_byte {
            b'7' | b'9' => &DP4,
            _ => &DP3,
        };
        result = (result * dp[count]) % MOD;

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_texts("22233".to_string()), 8);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_texts("222222222222222222222222222222222222".to_string()),
            82876089
        );
    }
}
