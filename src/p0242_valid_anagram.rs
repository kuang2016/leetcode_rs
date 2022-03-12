pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut a = [0; 26];

        for b in s.as_bytes() {
            let i = (b - b'a') as usize;
            a[i] += 1;
        }
        
        for b in t.as_bytes() {
            let i = (b - b'a') as usize;
            a[i] -= 1;

            if a[i] < 0 {
                return false;
            }
        }

        for i in 0..26 {
            if a[i] != 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert_eq!(false, Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
