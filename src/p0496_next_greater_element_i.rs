pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        let mut next: HashMap<i32, i32> = HashMap::new();

        for n in nums2 {
            while !stack.is_empty() && *stack.last().unwrap() < n {
                let t = stack.pop().unwrap();
                next.insert(t, n);
            }
            stack.push(n);
        }

        return nums1.iter().map(|n| *next.get(&n).unwrap_or(&-1)).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!(-1, 3, -1),
            Solution::next_greater_element(vec!(4, 1, 2), vec!(1, 3, 4, 2))
        );

        assert_eq!(
            vec!(3, -1),
            Solution::next_greater_element(vec!(2, 4), vec!(1, 2, 3, 4))
        );
    }
}
