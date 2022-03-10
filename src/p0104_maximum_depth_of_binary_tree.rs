pub struct Solution {}

use crate::data::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.unwrap();

        let l = Solution::max_depth(node.borrow_mut().left.take());
        let r = Solution::max_depth(node.borrow_mut().right.take());

        return l.max(r) + 1;
    }
}