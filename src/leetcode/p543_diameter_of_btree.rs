#![allow(dead_code)]

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

struct Solution;

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::max_depth(&root).1
    }

    fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(head) = root {
            let rf = head.borrow();
            let (r_depth, r_dia) = Self::max_depth(&rf.right);
            let (l_depth, l_dia) = Self::max_depth(&rf.left);
            let dia = l_depth + r_depth;

            let max_dia = cmp::max(l_dia, r_dia).max(dia);
            let max_depth = cmp::max(l_depth, r_depth);

            (max_depth + 1, max_dia)
        } else {
            (0, 0)
        }
    }
}
