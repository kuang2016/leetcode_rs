pub struct Solution {}

use crate::data::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            match root {
                None => Some(0),
                Some(n) => {
                    let l = dfs(n.borrow_mut().left.take());
                    if l.is_none() {
                        return None;
                    }
                    let r = dfs(n.borrow_mut().right.take());
                    if r.is_none() {
                        return None;
                    }

                    if (l.unwrap() - r.unwrap()).abs() > 1 {
                        return None;
                    }

                    return Some(l.unwrap().max(r.unwrap())+1);
                }
            }
        }

        return dfs(root).is_some();
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
