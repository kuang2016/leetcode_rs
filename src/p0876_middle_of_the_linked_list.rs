pub struct Solution {}

use crate::data::list::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        
        while let Some(node) = fast {
            match node.next.as_ref() {
                None => break,
                Some(fast_next) => {
                    fast = fast_next.next.as_ref();
                    slow = slow.unwrap().next.as_ref();
                }
            }
        }

        return slow.map(|n| n.clone());
    }
}