#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut res = 1;
        let mut set = std::collections::HashSet::<i32>::new();

        for n in nums {
            set.insert(n);
        }

        for n in set.iter() {
            // if no prev num, it is the starting num
            if let None = set.get(&(*n - 1)) {
                let mut i = 1;

                // have next num
                while let Some(_) = set.get(&(*n + i)) {
                    i += 1
                }

                res = std::cmp::max(res, i);
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
        9
    );
}
