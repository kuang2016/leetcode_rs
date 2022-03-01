pub struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let cnt = (salary.len() - 2) as f64;
        let mut min = f64::MAX;
        let mut max: f64 = 0.0;

        let mut sum = 0.0;

        for s in salary {
            let s = s as f64;
            min = min.min(s);
            max = max.max(s);
            sum += s;
        }

        sum = sum - min - max;

        return sum / cnt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2500.00000, Solution::average(vec![4000, 3000, 1000, 2000]));
        assert_eq!(2000.00000, Solution::average(vec![1000, 2000, 3000]));
    }
}
