pub struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));

        let n = nums.len();

        for i in 0..(n-2) {
            let a = nums[i];
            let b = nums[i+1];
            let c = nums[i+2];

            if a < b + c {
                return a + b + c;
            }
        }

        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::largest_perimeter(vec![2, 1, 2]));
        assert_eq!(0, Solution::largest_perimeter(vec![1, 2, 1]));
    }
}
