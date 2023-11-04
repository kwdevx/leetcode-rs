#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut res: i32 = 0;

        let mut hm = std::collections::HashMap::<&u8, i32>::with_capacity(26);
        let mut cur_max = 0;
        let mut l = 0;

        let chars = s.as_bytes();

        for r in 0..s.len() {
            *hm.entry(&chars[r]).or_default() += 1;

            cur_max = std::cmp::max(cur_max, hm[&chars[r]]);

            if (r - l + 1) as i32 - cur_max > k {
                *hm.entry(&chars[l]).or_default() -= 1;
                l += 1;
            }

            res = std::cmp::max(res, (r - l + 1) as i32);
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::character_replacement(String::from("ABAB"), 2), 4);
    assert_eq!(
        Solution::character_replacement(String::from("AABABBA"), 1),
        4
    )
}
