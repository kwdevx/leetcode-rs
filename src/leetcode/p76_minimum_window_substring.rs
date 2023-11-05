#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s == String::from("") {
            return String::from("");
        }

        let mut count_t = HashMap::<&u8, i32>::new();
        let mut window = HashMap::<&u8, i32>::new();

        for c in t.as_bytes().iter() {
            *count_t.entry(&c).or_default() += 1;
        }

        let mut res = (0, 0);
        let mut res_len = i32::MAX;
        let mut l = 0;
        let mut have = 0;
        let need = count_t.len();

        let s_bytes = s.as_bytes();
        for r in 0..s.len() {
            *window.entry(&s_bytes[r]).or_default() += 1;

            if count_t.contains_key(&s_bytes[r]) && count_t[&s_bytes[r]] == window[&s_bytes[r]] {
                have += 1;
            }

            while have == need {
                if ((r - l + 1) as i32) < res_len {
                    res = (l, r);
                    res_len = (r - l + 1) as i32;
                }

                *window.entry(&s_bytes[l]).or_default() -= 1;

                if count_t.contains_key(&s_bytes[l]) && window[&s_bytes[l]] < count_t[&s_bytes[l]] {
                    have -= 1
                }

                l += 1;
            }
        }

        let (res_l, res_r) = res;

        if res_len != i32::MAX {
            return (&s[res_l..res_r + 1]).to_string();
        } else {
            return String::from("");
        }
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
        String::from("BANC")
    );
}
