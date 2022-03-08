pub struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        for n in nums {
            if n == 0 {
                return 0;
            } else if n < 0 {
                ans *= -1;
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
        assert_eq!(1, Solution::array_sign(vec!(-1,-2,-3,-4,3,2,1)));
        assert_eq!(0, Solution::array_sign(vec!(1,5,0,2,-3)));
        assert_eq!(-1, Solution::array_sign(vec!(-1,1,-1,1,-1)));
    }
}
