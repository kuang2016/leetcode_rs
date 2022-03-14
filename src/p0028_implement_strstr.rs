pub struct Solution {}


impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        return haystack.find(&needle).map(|i| i as i32).unwrap_or(-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
        assert_eq!(0, Solution::str_str("hello".to_string(), "".to_string()));
        assert_eq!(-1, Solution::str_str("hello".to_string(), "abc".to_string()));
    }
}
