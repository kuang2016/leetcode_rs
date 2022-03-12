pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s = HashSet::new();

        for n in nums {
            if s.contains(&n) {
                return true;
            }
            s.insert(n);
        }
        return false;
    }
}