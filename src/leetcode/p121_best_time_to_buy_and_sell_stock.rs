#![allow(dead_code)]

use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // base case, return 0 if no profit
        let mut res = 0;

        if prices.len() <= 1 {
            return res;
        }

        let (mut l, mut r) = (0, 1);

        while l < r {
            let profit = prices[r] - prices[l];
            res = std::cmp::max(res, profit);

            if profit < 0 {
                l = r;
                if r != prices.len() - 1 {
                    r += 1;
                }
            } else {
                if r != prices.len() - 1 {
                    r += 1
                } else {
                    l += 1
                }
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
}
