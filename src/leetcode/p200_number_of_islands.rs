#![allow(dead_code)]

use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res: i32 = 0;
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let rows = grid.len();
        let cols = grid[0].len();

        println!("rows: {}, cols: {}", rows, cols);

        // eliminlate visited node
        let mut visited = HashSet::<(i32, i32)>::with_capacity(rows * cols);

        fn dfs(
            r: i32,
            c: i32,
            rows: i32,
            cols: i32,
            dirs: &Vec<(i32, i32)>,
            visited: &mut HashSet<(i32, i32)>,
            grid: &Vec<Vec<char>>,
        ) {
            // println!("checking r: {}, c: {}", r, c);
            //  check out of bound
            if r < 0 || c < 0 || r >= rows || c >= cols || grid[r as usize][c as usize] == '0' {
                // println!("out of bound!");
                return;
            }
            // check visited
            if let Some(..) = visited.get(&(r, c)) {
                // println!("visited!");
                return;
            } else {
                visited.insert((r, c));
            }

            // dfs for 4 directions
            for (dr, dc) in dirs {
                dfs(r + dr, c + dc, rows, cols, dirs, visited, grid);
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                match visited.get(&(row as i32, col as i32)) {
                    Some(..) => {}
                    None => {
                        if grid[row][col] == '1' {
                            dfs(
                                row as i32,
                                col as i32,
                                rows as i32,
                                cols as i32,
                                &dirs,
                                &mut visited,
                                &grid,
                            );

                            res += 1;
                        }
                    }
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
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]),
        1
    );
    // assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
