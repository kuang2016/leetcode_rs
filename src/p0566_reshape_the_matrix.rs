pub struct Solution {}

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        if (m * n) as i32 != r * c {
            return mat;
        }

        let mut a = vec![vec![0; c as usize]; r as usize];

        let mut i = 0;
        let mut j = 0;

        for x in 0..m {
            for y in 0..n {
                a[i][j] = mat[x][y];
                j += 1;
                if j == c as usize {
                    i += 1;
                    j = 0;
                }
            }
        }

        return a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![1,2,3,4]], Solution::matrix_reshape(vec![vec![1,2], vec![3,4]], 1, 4));
    }
}
