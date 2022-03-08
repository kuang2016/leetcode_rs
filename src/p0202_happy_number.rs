pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    fn get_next(mut n: i32) -> i32 {
        let mut ans = 0;

        while n > 0 {
            let d = n % 10;
            ans += d * d;
            n /= 10;
        }

        return ans;
    }

    pub fn is_happy(n: i32) -> bool {
        let mut k = n;
        let mut seen = HashSet::new();
        while k != 1 {
            if seen.contains(&k) {
                return false;
            } else {
                seen.insert(k);
                k = Solution::get_next(k);
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
        assert_eq!(true, Solution::is_happy(19));
        assert_eq!(false, Solution::is_happy(2));
    }
}
