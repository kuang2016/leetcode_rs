pub struct Solution {}


impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut ans = String::new();

        for c in s.chars() {
            if 'A' <= c && c <= 'Z' {
                ans.push((b'a' + (c as u8 - b'A')) as char);
            } else {
                ans.push(c);
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
        assert_eq!("lovely!", Solution::to_lower_case("Lovely!".to_string()));
    }
}
