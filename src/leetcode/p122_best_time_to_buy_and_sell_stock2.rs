#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        if prices.len() < 2 {
            return res;
        }

        for i in 1..prices.len() {
            if prices[i] - prices[i - 1] > 0 {
                res += prices[i] - prices[i - 1];
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
