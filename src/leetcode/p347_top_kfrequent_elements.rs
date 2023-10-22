#![allow(dead_code)]
use super::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == k as usize {
            return nums;
        }
        let mut count_s = vec![HashSet::<i32>::new(); nums.len()];

        let map = nums
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut acc, key| {
                *acc.entry(*key).or_default() += 1;
                acc
            });

        for (k, v) in map.iter() {
            count_s[(*v - 1) as usize].insert(*k);
        }

        let mut res: Vec<i32> = vec![];

        for i in (0..count_s.len()).rev() {
            for x in count_s[i].iter() {
                if res.len() == k as usize {
                    return res;
                } else {
                    res.push(*x);
                }
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
