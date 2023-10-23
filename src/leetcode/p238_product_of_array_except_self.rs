#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1i32; nums.len()];

        let mut prev = nums[0];
        for (i, num) in nums.iter().enumerate().skip(1) {
            res[i] = prev * res[i - 1];
            prev = *num;
        }

        prev = nums[nums.len() - 1];
        let mut postfix = 1;
        for (i, num) in nums.iter().enumerate().rev().skip(1) {
            postfix = prev * postfix;
            res[i] *= postfix;
            prev = *num;
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::product_except_self(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
    assert_eq!(
        Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}
