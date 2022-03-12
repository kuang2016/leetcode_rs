pub struct NumArray {
    a : Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut a = vec![0; nums.len()];
        a[0] = nums[0];
        for i in 1..nums.len() {
            a[i] = a[i-1] + nums[i];
        }

        Self {
            a : a,
        }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let x = if left == 0 {
            0
        } else {
            self.a[(left-1) as usize]
        };

        return self.a[right as usize] -x;
    }
}
