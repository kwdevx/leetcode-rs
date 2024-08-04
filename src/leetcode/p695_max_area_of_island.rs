#![allow(dead_code)]

use std::cmp::max;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let rows = grid.len();
        let cols = grid[0].len();

        let mut visited = HashSet::<(i32, i32)>::with_capacity(rows * cols);

        fn dfs(
            row: i32,
            col: i32,
            rows: i32,
            cols: i32,
            grid: &Vec<Vec<i32>>,
            dirs: &Vec<(i32, i32)>,
            visited: &mut HashSet<(i32, i32)>,
        ) -> i32 {
            if row < 0
                || col < 0
                || row >= rows
                || col >= cols
                || grid[row as usize][col as usize] == 0
            {
                0
            } else {
                match visited.get(&(row, col)) {
                    Some(..) => 0,
                    None => {
                        visited.insert((row, col));

                        let mut area = 1;

                        for (dr, dc) in dirs {
                            area += dfs(row + dr, col + dc, rows, cols, grid, dirs, visited);
                        }

                        area
                    }
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                let node = grid[row][col];
                if node == 1 {
                    let area = dfs(
                        row as i32,
                        col as i32,
                        rows as i32,
                        cols as i32,
                        &grid,
                        &dirs,
                        &mut visited,
                    );

                    res = max(res, area)
                }
            }
        }

        res
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::max_area_of_island(vec![
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0].to_vec(),
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0].to_vec(),
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0].to_vec(),
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0].to_vec(),
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0].to_vec()
        ]),
        6
    );
    assert_eq!(
        Solution::max_area_of_island(vec![
            [1, 1, 0, 0, 0].to_vec(),
            [1, 1, 0, 0, 0].to_vec(),
            [0, 0, 0, 1, 1].to_vec(),
            [0, 0, 0, 1, 1].to_vec()
        ]),
        4
    );

    let x: usize = 0;
    assert_eq!(x - 1, 0);
}
