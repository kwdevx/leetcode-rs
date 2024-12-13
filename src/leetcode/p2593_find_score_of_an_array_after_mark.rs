#![allow(dead_code)]
struct Solution {}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut visited = HashSet::<i32>::new();

        let sorted = nums
            .iter()
            .enumerate()
            .map(|(i, x)| (Reverse(*x), Reverse(i as i32)))
            .collect::<Vec<(Reverse<i32>, Reverse<i32>)>>();

        let mut hp = BinaryHeap::from(sorted);

        let mut res: i64 = 0;
        while !hp.is_empty() {
            let (Reverse(val), Reverse(idx)) = hp.pop().unwrap();
            println!("value: {}, idx: {}", val, idx);

            if !visited.contains(&idx) {
                res += val as i64;

                if (idx as i32 - 1) >= 0 {
                    visited.insert(idx - 1);
                }
                if (idx as i32) + 1 < nums.len() as i32 {
                    visited.insert(idx + 1);
                }
            }
        }

        res
    }
    pub fn find_score_2(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();

        nums.sort_by_key(|&(i, x)| (x, i));

        let mut bucket = vec![false; nums.len()];
        nums.iter().fold(0_i64, |acc, &(i, x)| {
            if bucket[i] {
                acc
            } else {
                bucket[i] = true;
                if i > 0 {
                    bucket[i - 1] = true;
                }
                if i + 1 < nums.len() {
                    bucket[i + 1] = true;
                }

                acc + x as i64
            }
        })
    }
}

#[cfg(test)]
#[test]
fn main() {
    // assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
    // assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);

    use std::cmp::Ordering;
    // assert_eq!(Solution::find_score(vec![5, 1, 1, 7, 2, 4]), 3);
    assert_eq!([2, 1].iter().cmp([1, 2].iter()), Ordering::Greater);
}
