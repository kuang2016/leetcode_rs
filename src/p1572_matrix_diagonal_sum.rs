pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();

        let mut ans = 0;
        for i in 0..n {
            ans += mat[i][i];
            ans += mat[i][n-1-i];
        }

        if n % 2 == 1 {
            let j = (n-1) / 2;
            ans -= mat[j][j];
        }

        return ans;
    }
}
