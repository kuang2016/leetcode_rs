pub struct Solution {}

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = 0;

        let mut s = Vec::new();

        while j < popped.len() {
            while i < pushed.len() && pushed[i] != popped[j] {
                s.push(pushed[i]);
                i += 1;
            }

            if i == pushed.len() {
                return false;
            } else {
                i += 1;
                j += 1;
            }

            while j < popped.len() && !s.is_empty() && popped[j] == *s.last().unwrap() {
                s.pop();
                j += 1;
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
        assert_eq!(true, Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]));
        assert_eq!(false, Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]));
    }
}
