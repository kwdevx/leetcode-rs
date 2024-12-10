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

struct Solution {}
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(head) = root {
            let mut res = 0;
            let mut q = VecDeque::new();
            q.push_back(head);

            while !q.is_empty() {
                let level_size = q.len();
                res += 1;

                for _ in 0..level_size {
                    if let Some(node) = q.pop_front() {
                        let rf = node.as_ref().borrow();
                        match (rf.left.clone(), rf.right.clone()) {
                            (Some(left), Some(right)) => {
                                q.push_back(left);
                                q.push_back(right);
                            }
                            (Some(left), None) => {
                                q.push_back(left);
                            }
                            (None, Some(right)) => {
                                q.push_back(right);
                            }
                            (None, None) => {}
                        }
                    }
                }
            }

            res
        } else {
            0
        }
    }
}
