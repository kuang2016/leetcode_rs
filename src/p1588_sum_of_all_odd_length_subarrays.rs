pub struct Solution {}


impl Solution {
    pub fn sum_odd_length_subarrays(a: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = a.len();
        let m = if n % 2 == 0 {n/2} else {n/2+1};


        let mut cache = vec![vec![0; n]; m+1];

        for i in 0..n {
            cache[1][i] = a[i];
            ans += a[i];
        }

        for k in 2..(m+1) {
            let l = (k-1) * 2 + 1;
            for i in 0..n-l+1 {
                cache[k][i] = cache[k-1][i] + a[i+l-1] + a[i+l-2];
                ans += cache[k][i];
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
        assert_eq!(58, Solution::sum_odd_length_subarrays(vec![1,4,2,5,3]));
        assert_eq!(3, Solution::sum_odd_length_subarrays(vec![1,2]));
        assert_eq!(66, Solution::sum_odd_length_subarrays(vec![10,11,12]));
    }
}
