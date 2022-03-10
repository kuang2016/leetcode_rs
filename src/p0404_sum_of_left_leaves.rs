pub struct Solution {}
use crate::data::tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_of_left_leaves(root: Node) -> i32 {
        fn helper(root: &Node, is_left: bool) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let node = node.borrow();
                    let (left, right) = (&node.left, &node.right);
                    if left.is_none() && right.is_none() {
                        if is_left { node.val } else { 0 }
                    } else {
                        helper(left, true) + helper(right, false)
                    }
                }
            }
        } 
        helper(&root, false)
    }
}
