#![allow(dead_code)]
struct Solution {}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 3 as usize {
            if nums.iter().sum::<i32>() == 0 {
                return vec![nums];
            } else {
                return vec![];
            }
        }

        // let mut nums = nums.clone();
        nums.sort();
        let mut res = vec![];

        for i in 0..nums.len() {
            let num = nums[i];
            if (i - 1) as i32 != -1 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, nums.len() - 1);

            while l < r {
                let sum_of_two = nums[l] + nums[r];
                let target = -num;

                if sum_of_two > target {
                    r -= 1
                } else if sum_of_two < target {
                    l += 1
                } else {
                    res.push(vec![num, nums[l], nums[r]]);
                    l += 1;
                    while nums[l] == nums[l - 1] && l < r {
                        l += 1
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
}
