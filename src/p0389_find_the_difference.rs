pub struct Solution {}


impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut x = [0; 26];
        let mut y = [0; 26];

        for b in s.as_bytes() {
            let k = b - ('a' as u8);
            x[k as usize] += 1;
        }

        for b in t.as_bytes() {
            let k = b - ('a' as u8);
            y[k as usize] += 1;
        }       

        for i in 0..26 {
            if x[i] != y[i] {
                return ('a' as u8 + i as u8) as char;
            }
        }

        return ' ';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!('e', Solution::find_the_difference("abcd".to_string(), "abcde".to_string()));
        assert_eq!('y', Solution::find_the_difference("".to_string(), "y".to_string()));
    }
}
