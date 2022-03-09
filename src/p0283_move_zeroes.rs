pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut l = 0;

        for i in 0..n {
            if nums[i] != 0 {
                if i != l {
                    nums[l] = nums[i];
                    nums[i] = 0;
                }
                l += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut a = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut a);
        assert_eq!(vec![1, 3, 12, 0, 0], a);
    }

    #[test]
    fn test_2() {
        let mut a = vec![0];
        Solution::move_zeroes(&mut a);
        assert_eq!(vec![0], a);
    }

    #[test]
    fn test_3() {
        let mut a = vec![1];
        Solution::move_zeroes(&mut a);
        assert_eq!(vec![1], a);
    }

    #[test]
    fn test_4() {
        let mut a = vec![2, 1];
        Solution::move_zeroes(&mut a);
        assert_eq!(vec![2, 1], a);
    }
}
