#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut res = 0;
        let mut l = 0;

        for r in 0..nums.len() {
            while nums[r] - k > nums[l] + k {
                l += 1;
            }
            res = std::cmp::max(res, r - l + 1);
        }

        res as i32
    }
}
