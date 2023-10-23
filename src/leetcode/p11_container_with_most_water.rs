#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0i32;

        let (mut l, mut r) = (0, height.len() - 1);

        while l < r {
            let vol = std::cmp::min(height[l], height[r]) * (r - l) as i32;
            res = std::cmp::max(res, vol);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
