#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // if the prefix sum different of from and to is not changed, it is a special substring
        // fill first index with a dummy 0
        let prefix: Vec<i32> = std::iter::once(0)
            .chain(
                nums.windows(2)
                    .map(|ele| (ele[0] & 1 == ele[1] & 1) as i32)
                    .scan(0, |acc, ele| {
                        *acc += ele;
                        Some(*acc)
                    }),
            )
            .collect();

        queries
            .iter()
            .enumerate()
            .fold(vec![true; queries.len()], |mut acc, (i, q)| {
                acc[i] = (prefix[q[1] as usize] - prefix[q[0] as usize]) == 0;
                acc
            })
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::is_array_special(vec![3, 4, 1, 2, 6], vec![vec![0, 4]]),
        vec![false]
    );

    // assert_eq!(
    //     Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]]),
    //     vec![false, true]
    // );
}
