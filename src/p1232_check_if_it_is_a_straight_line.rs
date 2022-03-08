pub struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let dx = coordinates[1][0] - coordinates[0][0];
        let dy = coordinates[1][1] - coordinates[0][1];

        for i in 2..coordinates.len() {
            let dx1 = coordinates[i][0] - coordinates[i-1][0];
            let dy1 = coordinates[i][1] - coordinates[i-1][1];

            if dy * dx1 != dx * dy1 {
                return false;
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
        assert_eq!(2, 1+1);
    }
}
