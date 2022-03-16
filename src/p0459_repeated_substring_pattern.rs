pub struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut x = String::new();
        x.push_str(&s);
        x.push_str(&s);
        return x[1..x.len()-1].contains(&s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::repeated_substring_pattern("abab".to_string()));
    }
}
