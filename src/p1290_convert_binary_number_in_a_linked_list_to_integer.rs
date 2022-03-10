pub struct Solution {}

use crate::data::list::ListNode;


impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut p = head;

        let mut ans = 0;
        while p.is_some() {
            let node = *p.unwrap();

            ans = ans * 2 + node.val;
            p = node.next;
        }
        return ans;
    }
}