#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(x) = map.get(v) {
                return vec![*x, i as i32];
            }

            map.insert(target - v, i as i32);
        }

        // base case
        return vec![0, 1];
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
