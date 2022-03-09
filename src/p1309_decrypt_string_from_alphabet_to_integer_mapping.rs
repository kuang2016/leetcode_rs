use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let m = HashMap::from([
            ("1", 'a'),
            ("2", 'b'),
            ("3", 'c'),
            ("4", 'd'),
            ("5", 'e'),
            ("6", 'f'),
            ("7", 'g'),
            ("8", 'h'),
            ("9", 'i'),
            ("10#", 'j'),
            ("11#", 'k'),
            ("12#", 'l'),
            ("13#", 'm'),
            ("14#", 'n'),
            ("15#", 'o'),
            ("16#", 'p'),
            ("17#", 'q'),
            ("18#", 'r'),
            ("19#", 's'),
            ("20#", 't'),
            ("21#", 'u'),
            ("22#", 'v'),
            ("23#", 'w'),
            ("24#", 'x'),
            ("25#", 'y'),
            ("26#", 'z'),
        ]);

        let mut ans = String::new();

        let mut i = 0;
        let bs = s.as_bytes();

        while i < s.len() {
            let (j, key) = if i + 2 < s.len() && bs[i + 2] == b'#' {
                (i+3, s[i..i+3].to_string())
            } else {
                (i+1, s[i..i+1].to_string())
            };

            ans.push(*m.get(key.as_str()).unwrap());

            i = j;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("jkab", Solution::freq_alphabets("10#11#12".to_string()));
    }
}
