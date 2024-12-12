#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut hp = std::collections::BinaryHeap::<i32>::from(gifts);

        for _ in 0..k {
            if let Some(largest) = hp.pop() {
                let left = (largest as f64).sqrt();
                hp.push(left as i32);
            }
        }

        hp.into_iter().map(|a| a as i64).sum::<i64>()
    }
}
