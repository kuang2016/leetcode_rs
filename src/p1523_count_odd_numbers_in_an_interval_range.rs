pub struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let n = high - low + 1;

        return match (low % 2, high % 2) {
            (0, 0) => (n - 1) / 2,
            (0, 1) => (n - 2) / 2 + 1,
            (1, 0) => n / 2,
            _ => (n - 1) / 2 + 1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::count_odds(0, 0));
        assert_eq!(1, Solution::count_odds(0, 1));
        assert_eq!(3, Solution::count_odds(3, 7));
        assert_eq!(1, Solution::count_odds(8, 10));
    }
}
