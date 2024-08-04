#![allow(dead_code)]

use std::collections::HashSet;
struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = heights.len();
        let cols = heights[0].len();
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut pac = HashSet::<(i32, i32)>::with_capacity(rows * cols);
        let mut alt = HashSet::<(i32, i32)>::with_capacity(rows * cols);
        let mut ans: Vec<Vec<i32>> = vec![];

        fn dfs(
            r: i32,
            c: i32,
            dirs: &Vec<(i32, i32)>,
            ocean: &mut HashSet<(i32, i32)>,
            heights: &Vec<Vec<i32>>,
            prev: i32,
        ) {
            if r < 0 || c < 0 || r >= heights.len() as i32 || c >= heights[0].len() as i32 {
                return;
            }
            if let Some(..) = ocean.get(&(r, c)) {
                return;
            }
            // check visited
            for (dr, dc) in dirs {
                // check out of bound
                let node = heights[r as usize][c as usize];
                if node < prev {
                    continue;
                } else {
                    ocean.insert((r, c));
                    dfs(r + dr, c + dc, dirs, ocean, heights, node);
                }
            }
        }

        for r in 0..rows {
            dfs(r as i32, 0 as i32, &dirs, &mut pac, &heights, 0);
            dfs(r as i32, cols as i32 - 1, &dirs, &mut alt, &heights, 0);
        }
        for c in 0..cols {
            dfs(0 as i32, c as i32, &dirs, &mut pac, &heights, 0);
            dfs(rows as i32 - 1, c as i32, &dirs, &mut alt, &heights, 0);
        }

        for r in 0..rows {
            for c in 0..cols {
                match (
                    pac.get(&(r as i32, c as i32)),
                    alt.get(&(r as i32, c as i32)),
                ) {
                    (Some(..), Some(..)) => {
                        ans.push(vec![r as i32, c as i32]);
                    }
                    _ => {}
                };
            }
        }

        ans
    }
}

#[cfg(test)]
#[test]
fn main() {
    // assert_eq!(Solution::pacific_atlantic(String::from("ABAB"), 2), 4);
}
