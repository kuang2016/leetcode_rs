pub struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        return accounts.iter().map(|banks| (*banks).iter().sum()).max().unwrap();
    }
}
