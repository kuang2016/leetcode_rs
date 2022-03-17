pub struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        fn paired(s: &[u8], i: usize) -> usize {
            let mut x = 0;
            let mut j = i;
            while j < s.len() {
                if s[j] == b'(' {
                    x += 1;
                } else {
                    x -= 1;
                }

                if x == 0 {
                    return j;
                }

                j += 1;
            }
            return 0;
        }

        fn calc(b: &[u8]) -> usize {
            if b.len() == 2 && b[0] == b'(' && b[1] == b')' {
                return 1;
            }

            let mut i = 0;
            let mut j = 0;
            let mut acc = 0;
            while j < b.len() {
                j = paired(b, i);
                if j == i+1 {
                    acc += 1;
                } else {
                    let t = calc(&b[i+1..j]);
                    acc += t * 2;
                }
                i = j + 1;
                j = i;
            }

            return acc;
        }

        return calc(s.as_bytes()) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::score_of_parentheses("()".to_string()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::score_of_parentheses("()()".to_string()));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::score_of_parentheses("(())".to_string()));
        assert_eq!(4, Solution::score_of_parentheses("((()))".to_string()));
        assert_eq!(7, Solution::score_of_parentheses("((()))(())()".to_string()));
    }
}
