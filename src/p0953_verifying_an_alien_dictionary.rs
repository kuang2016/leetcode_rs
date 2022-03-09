use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let d: HashMap<char, usize> = order.chars().enumerate().map(|(i, c)| (c, i)).collect();

        for i in 1..words.len() {
            let x = &words[i-1];
            let y = &words[i];

            for i in 0..x.len() {
                if i >= y.len() { 
                    return false;
                }
                let p = *d.get(&x.chars().nth(i).unwrap()).unwrap(); 
                let q = *d.get(&y.chars().nth(i).unwrap()).unwrap(); 
                if p < q {
                    break;
                } else if p > q {
                    return false;
                }
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
        assert_eq!(true, Solution::is_alien_sorted(vec!["hello", "leetcode"].iter().map(|s| s.to_string()).collect(), "hlabcdefgijkmnopqrstuvwxyz".to_string()));
        assert_eq!(false, Solution::is_alien_sorted(vec!["word","world","row"].iter().map(|s| s.to_string()).collect(), "worldabcefghijkmnpqstuvxyz".to_string()));
        
    }
}
