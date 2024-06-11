// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                None => (0, 0),
                Some(n) => {
                    let n = n.borrow();
                    let (l, x) = depth(&n.left);
                    let (r, y) = depth(&n.right);
                    let d = x.max(y).max(l + r);
                    if l > r {
                        (l + 1, d)
                    } else {
                        (r + 1, d)
                    }
                }
            }
        }
        depth(&root).1
    }
}
