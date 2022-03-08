pub struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(mut a: Vec<i32>) -> bool {
        a.sort();
        let diff = (a[0] - a[1]).abs();
        return (2..a.len()).all(|i| (a[i] - a[i-1]).abs() == diff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_make_arithmetic_progression(vec!(3,5,1)));
    }
}
