pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        x = ((x & 0xffff0000) >> 16) | ((x & 0x0000ffff) << 16);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
        ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            3221225471
        );
    }
}
