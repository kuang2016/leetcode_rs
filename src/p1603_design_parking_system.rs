pub struct ParkingSystem {
    spaces: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: vec![0, big, medium, small]
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        if self.spaces[car_type as usize] > 0 {
            self.spaces[car_type as usize] -= 1;
            return true;
        } else {
            return false;
        }
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
