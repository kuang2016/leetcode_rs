pub struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let mut ans = points.len();

        for (i, pair) in points.iter().enumerate() {
            let (a, b) = (pair[0], pair[1]);
            if x == a || y == b {
                let t = (x-a).abs() + (y-b).abs();
                if t < min {
                    min = t;
                    ans = i;
                }
            }
        }

        if ans == points.len() {
            return -1;
        } else {
            return ans as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::nearest_valid_point(3, 4, vec![vec![1,2],vec![3,1],vec![2,4],vec![2,3],vec![4,4]]));
        assert_eq!(0, Solution::nearest_valid_point(3, 4, vec![vec![3,4]]));
    }
}
