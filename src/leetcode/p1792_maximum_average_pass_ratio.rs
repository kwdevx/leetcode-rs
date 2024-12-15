#![allow(dead_code)]

struct Solution;

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(PartialEq)]
struct NonNan(f64);

impl Eq for NonNan {}
impl PartialOrd for NonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut max_heap = BinaryHeap::new();

        let mut res = 0.0;

        for x in &classes {
            let pass = x[0] as f64;
            let total = x[1] as f64;
            let gain = Self::gain(pass, total);
            res += pass / total;
            max_heap.push((NonNan(gain), NonNan(pass), NonNan(total)));
        }

        (0..extra_students).into_iter().for_each(|_| {
            if !max_heap.is_empty() {
                let (_, pass, total) = max_heap.pop().unwrap();
                res -= pass.0 / total.0;
                let new_pass = pass.0 + 1.0;
                let new_total = total.0 + 1.0;
                let new_gain = Self::gain(new_pass, new_total);
                res += new_pass / new_total;
                max_heap.push((NonNan(new_gain), NonNan(new_pass), NonNan(new_total)));
            }
        });

        res / (classes.len() as f64)
    }

    fn gain(pass: f64, total: f64) -> f64 {
        (pass + 1.0) / (total + 1.0) - (pass / total)
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
        0.78333
    );
}
