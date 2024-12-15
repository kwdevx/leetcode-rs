#![allow(dead_code)]
struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let (mut l, mut res) = (0, 0);
        let (mut min_q, mut max_q) = (BinaryHeap::new(), BinaryHeap::new());

        for r in 0..nums.len() {
            min_q.push((Reverse(nums[r]), r));
            max_q.push((nums[r], r));

            while min_q.peek().is_some()
                && max_q.peek().is_some()
                && (max_q.peek().unwrap().0 - (min_q.peek().unwrap().0 .0)) > 2
            {
                l += 1;
                while !max_q.is_empty() && max_q.peek().unwrap().1 < l {
                    max_q.pop();
                }
                while !min_q.is_empty() && min_q.peek().unwrap().1 < l {
                    min_q.pop();
                }
            }

            res += (r - l + 1) as i64;
        }

        res
    }
}
