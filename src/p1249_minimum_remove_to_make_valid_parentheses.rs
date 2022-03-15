pub struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut l = Vec::new();
        let mut r = Vec::new();

        let mut ans = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => l.push(i),
                ')' => if l.is_empty() {
                    r.push(i);
                } else {
                    l.pop();
                }
                _ => continue
            }
        }

        let mut i = 0;
        let mut j = 0;

        for (k, c) in s.chars().enumerate() {
            if i < l.len() && l[i] == k {
                i += 1;
            } else if j < r.len() && r[j] == k {
                j += 1;
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
        assert_eq!("lee(t(c)o)de", Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()));
        assert_eq!("ab(c)d", Solution::min_remove_to_make_valid("a)b(c)d".to_string()));
        assert_eq!("", Solution::min_remove_to_make_valid("))((".to_string()));
    }
}
