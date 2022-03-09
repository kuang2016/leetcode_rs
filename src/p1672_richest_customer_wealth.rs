pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        return accounts.iter().map(|banks| (*banks).iter().sum()).max().unwrap();
    }
}
