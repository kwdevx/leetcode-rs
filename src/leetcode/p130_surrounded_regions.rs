#![allow(dead_code)]

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut visited = HashSet::<(i32, i32)>::new();
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn dfs(
            r: i32,
            c: i32,
            dirs: &Vec<(i32, i32)>,
            board: &mut Vec<Vec<char>>,
            visited: &mut HashSet<(i32, i32)>,
        ) {
            if r < 0
                || c < 0
                || r as usize >= board.len()
                || c as usize >= board[0].len()
                || board[r as usize][c as usize] == 'X'
            {
                return;
            }
            match visited.get(&(r, c)) {
                Some(..) => {
                    return;
                }
                None => {
                    visited.insert((r, c));
                    board[r as usize][c as usize] = 'M';
                    for (dr, dc) in dirs {
                        dfs(r + dr, c + dc, dirs, board, visited);
                    }
                }
            }
        }

        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if (r == 0 || c == 0 || r == board.len() - 1 || c == board[0].len() - 1)
                    && board[r][c] == 'O'
                {
                    dfs(r as i32, c as i32, &dirs, board, &mut visited)
                }
            }
        }

        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == 'M' {
                    board[r][c] = 'O';
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn main() {}
