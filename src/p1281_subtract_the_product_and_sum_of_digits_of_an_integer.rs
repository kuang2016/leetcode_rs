pub struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut m = n;
        let mut s = 0;
        let mut p = 1;

        while m > 0 {
            let d = m % 10;
            m = m / 10;

            s += d;
            p *= d;
        }

        return p - s;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(15, Solution::subtract_product_and_sum(234));
        assert_eq!(21, Solution::subtract_product_and_sum(4421));
    }
}
