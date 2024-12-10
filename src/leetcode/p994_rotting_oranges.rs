#![allow(dead_code)]

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)].to_vec();

        let mut level = 0;
        let mut fresh = 0;
        let mut q = VecDeque::<(i32, i32)>::with_capacity(rows * cols);

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 2 {
                    q.push_back((r as i32, c as i32));
                } else if grid[r][c] == 1 {
                    fresh += 1
                }
            }
        }

        while q.len() > 0 && fresh > 0 {
            let this_level_nodes = q.len();

            for _ in 0..this_level_nodes {
                match q.pop_front() {
                    Some((r, c)) => {
                        for (dr, dc) in &dirs {
                            let (nr, nc) = (r + dr, c + dc);

                            if nr < 0
                                || nc < 0
                                || nr >= rows as i32
                                || nc >= cols as i32
                                || grid[nr as usize][nc as usize] != 1
                            {
                                continue;
                            }

                            if grid[nr as usize][nc as usize] == 1 {
                                grid[nr as usize][nc as usize] = 2;
                                fresh -= 1;
                                q.push_back((nr, nc))
                            }
                        }
                    }
                    None => {}
                }
            }

            if q.len() > 0 {
                level += 1;
            }
        }

        if fresh == 0 {
            level
        } else {
            -1
        }
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::oranges_rotting(vec![
            [2, 1, 1].to_vec(),
            [1, 1, 0].to_vec(),
            [0, 1, 1].to_vec()
        ]),
        4
    );
}
