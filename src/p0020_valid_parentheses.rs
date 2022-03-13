pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut t: Vec<u8> = Vec::new();
        for b in s.as_bytes() {
            if *b == b'(' || *b == b'[' || *b == b'{' {
                t.push(*b);
            } else {
                if t.is_empty() {
                    return false;
                }

                if *b == b')' && *t.last().unwrap() == b'(' ||
                 *b == b']' && *t.last().unwrap() == b'[' ||
                 *b == b'}' && *t.last().unwrap() == b'{' {
                     t.pop();
                 } else {
                     return false;
                 }
            }
        }
        return t.is_empty();
    }
}