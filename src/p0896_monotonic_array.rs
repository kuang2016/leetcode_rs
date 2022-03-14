use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        match nums.len() {
            1 => return true,
            _ => {
                let mut cmp = match nums[1].cmp(&nums[0]) {
                    std::cmp::Ordering::Equal => None,
                    x => Some(x)
                };
                for i in 1..nums.len() {
                    let tmp = nums[i].cmp(&nums[i-1]);

                    match (cmp, tmp) {
                        (_, Ordering::Equal) => continue,
                        (Some(c), t) => if t != c { return false; },
                        (None, t) => {
                            cmp = Some(t);
                            continue
                        }
                    }
                }
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_monotonic(vec![1,2,2,3]));
        assert_eq!(true, Solution::is_monotonic(vec![6,5,4,4]));
        assert_eq!(false, Solution::is_monotonic(vec![1,3,2]));
    }
}
