pub struct Solution {}

impl Solution {
    #![allow(non_snake_case)]
    pub fn hammingWeight (n: u32) -> i32 {
        let mut ans = 0;

        let mut m = n;
        while m > 0 {
            if m % 2 == 1 {
                ans += 1;
            }
            m = m / 2;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::hammingWeight(0b00000000000000000000000000001011));
        assert_eq!(1, Solution::hammingWeight(0b00000000000000000000000010000000));
        assert_eq!(31, Solution::hammingWeight(0b11111111111111111111111111111101));
    }
}
