#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let max_window_len = s.len() - 2;

        let mut cur_window_size = max_window_len;

        let mut hm = std::collections::HashMap::<&str, i32>::new();
        while cur_window_size > 0 {
            let mut l = 0;
            let mut r = cur_window_size;
            while r <= s.len() {
                let str = &s[l..r];
                if Self::is_special(str) {
                    *hm.entry(str).or_insert(0) += 1;
                }

                r += 1;
                l += 1;
            }

            cur_window_size -= 1;
        }

        if let Some(temp) = hm
            .iter()
            .filter(|(_, v)| **v >= 3)
            // .inspect(|x| {
            //     println!("key: {}, val: {}", x.0, x.1);
            // })
            .max_by_key(|x| x.0.len())
        {
            temp.0.len() as i32
        } else {
            -1
        }
    }

    fn is_special(s: &str) -> bool {
        let s_bytes = s.as_bytes();

        for i in 0..s_bytes.len() - 1 {
            if s_bytes[i] != s_bytes[i + 1] {
                return false;
            }
        }

        true
    }

    pub fn maximum_length_2(s: String) -> i32 {
        let n = s.len();

        let mut l = 0;
        let mut r = n;
        let bytes = s.as_bytes();

        // Check if no valid solution exists
        if !Self::is_valid(bytes, n, l) {
            return -1;
        }

        while l + 1 < r {
            let mid = (r + l) / 2;
            if Self::is_valid(bytes, n, mid) {
                l = mid;
            } else {
                r = mid;
            }
        }

        l as i32
    }

    fn is_valid(s: &[u8], n: usize, window_size: usize) -> bool {
        let mut l = 0;
        let mut bucket = vec![0; 26];

        for r in 0..n {
            while s[l] != s[r] {
                l += 1;
            }

            if r - l + 1 >= window_size {
                let idx = (s[r] - b'a') as usize;
                bucket[idx] += 1;
                if bucket[idx] > 2 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
#[test]
fn main() {
    // assert_eq!(
    //     Solution::maximum_length_2(String::from(
    //         "cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde"
    //     )),
    //     2
    // );
    assert_eq!(Solution::maximum_length_2(String::from("aaabbaaa")), 2);
}
