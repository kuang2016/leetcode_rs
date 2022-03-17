pub struct Solution {}

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();

        let mut cur = 0;
        for c in s.chars() {
            match c {
                '(' => { 
                    stack.push(cur);
                    cur = 0;
                },
                _ => {
                    cur = stack.pop().unwrap() + 1.max(cur * 2);
                }
            }
        }

        return cur;
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
    }

    #[test]
    fn test_4() {
        assert_eq!(4, Solution::score_of_parentheses("((()))".to_string()));
    }

    #[test]
    fn test_5() {
        assert_eq!(7, Solution::score_of_parentheses("((()))(())()".to_string()));
    }
}
