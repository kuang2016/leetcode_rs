pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = HashMap::new();

        let mut head = 0;
        let mut ans = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(last) = m.insert(c, i) {
                head = head.max(last+1);
            }
            ans = ans.max(i - head + 1);
        }

        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::length_of_longest_substring("".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("1".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::length_of_longest_substring("umvejcuuk".to_string()));
    }
}
