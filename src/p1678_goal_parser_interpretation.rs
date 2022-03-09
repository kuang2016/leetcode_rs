pub struct Solution {}


impl Solution {
    pub fn interpret(command: String) -> String {
        // 0 - init
        // 1 - met (
        // 2 - in 'al'
        let mut state = 0;
        let mut ans = String::new();

        for c in command.chars() {
            match c {
                'G' => ans.push(c),
                '(' => state = 1,
                ')' => if state == 1 { ans.push('o') } else { ans.push_str("al")},
                _ => state = 2,
            }
        }

        return ans;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("Goal", Solution::interpret("G()(al)".to_string()));
        assert_eq!("Gooooal", Solution::interpret("G()()()()(al)".to_string()));
        assert_eq!("alGalooG", Solution::interpret("(al)G(al)()()G".to_string()));
    }
}
