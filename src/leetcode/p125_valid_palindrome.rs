#![allow(dead_code)]
use super::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        // two pointers
        //
        // if s.len() == 0 {
        //     return true;
        // }

        // let (mut left, mut right) = (0, s.len() - 1);

        // while left < right {
        //     if s.get(left) != s.get(right) {
        //         return false;
        //     }
        //     left += 1;
        //     right -= 1;
        // }

        // functional

        for (c1, c2) in s.iter().zip(s.iter().rev()) {
            if c1 != c2 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
        true
    );
    assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
    assert_eq!(Solution::is_palindrome(String::from(" ")), true);
}
