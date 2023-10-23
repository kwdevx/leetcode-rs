#![allow(dead_code)]

use super::Solution;

impl Solution {
    //  map and set solution
    // pub fn first_uniq_char(s: String) -> i32 {
    //     let mut repeated = std::collections::HashSet::<char>::new();
    //     let mut non_repeated = std::collections::HashMap::<char, usize>::new();

    //     for (i, v) in s.chars().enumerate() {
    //         if let Some(_) = non_repeated.get(&v) {
    //             non_repeated.remove(&v);
    //             repeated.insert(v);
    //         } else if let None = repeated.get(&v) {
    //             non_repeated.insert(v, i);
    //         }
    //     }

    //     let mut min_index = s.len();

    //     for (_, v) in non_repeated.iter() {
    //         if *v < min_index {
    //             min_index = *v;
    //         }
    //     }

    //     if min_index == s.len() {
    //         return -1;
    //     } else {
    //         return min_index as i32;
    //     }
    // }

    pub fn first_uniq_char(s: String) -> i32 {
        let mut bucket = vec![0; 26];

        for (_, c) in s.chars().enumerate() {
            bucket[((c as u8) - ('a' as u8)) as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if bucket[((c as u8) - ('a' as u8)) as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(Solution::first_uniq_char(String::from("leetcode")), 0);
    assert_eq!(Solution::first_uniq_char(String::from("loveleetcode")), 2);
    assert_eq!(Solution::first_uniq_char(String::from("aabb")), -1);
}
