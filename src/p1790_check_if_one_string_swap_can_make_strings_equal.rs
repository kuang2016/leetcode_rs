pub struct Solution {}

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {

        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();

        let mut diff = 0;
        let mut j = 0; 
        for i in 0..s1.len() {
            if b1[i] != b2[i] {
                diff += 1;
                if diff == 1 {
                    j = i;
                } else if diff == 2 {
                    if b1[i] != b2[j] || b1[j] != b2[i] {
                        return false;
                    }
                } else {
                    return false;
                }

            }
        }
        return diff == 0 || diff == 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::are_almost_equal("bank".to_string(), "kanb".to_string()));
        assert_eq!(false, Solution::are_almost_equal("abcd".to_string(), "dcba".to_string()));
        assert_eq!(false, Solution::are_almost_equal("ac".to_string(), "aa".to_string()));
        assert_eq!(true, Solution::are_almost_equal("abc".to_string(), "abc".to_string()));
    }
}
