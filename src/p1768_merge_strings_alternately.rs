pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = String::new();

        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();

        let mut i = 0;
        while i < word1.len() && i < word2.len() {
            ans.push(b1[i] as char);
            ans.push(b2[i] as char);
            i += 1;
        }

        if i < word1.len() {
            ans.push_str(&word1[i..]);
        } else if i < word2.len() {
            ans.push_str(&word2[i..]);
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("apbqcr", Solution::merge_alternately("abc".to_string(), "pqr".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!("apbqrs", Solution::merge_alternately("ab".to_string(), "pqrs".to_string()));
    }
}
