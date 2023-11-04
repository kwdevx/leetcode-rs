#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l: usize = 0;
        let mut res: i32 = 0;
        let mut char_set = std::collections::HashSet::<u8>::new();

        let chars: &[u8] = s.as_bytes();

        for (r, cr) in chars.iter().enumerate() {
            while let Some(_) = char_set.get(cr) {
                char_set.remove(&(chars[l]));
                l += 1;
            }

            char_set.insert(*cr);

            res = std::cmp::max(res, (r - l + 1) as i32);
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbb")),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    )
}
