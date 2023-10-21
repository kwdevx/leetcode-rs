#![allow(dead_code)]
use super::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());

        for num in &nums {
            if !set.insert(num) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        true
    );
}
