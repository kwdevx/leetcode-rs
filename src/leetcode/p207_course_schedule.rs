#![allow(dead_code)]

struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let hm = &mut HashMap::<i32, Vec<i32>>::new();

        for prereq in prerequisites {
            let (cur, pre) = (prereq[0], prereq[1]);
            hm.entry(cur).or_insert(vec![]).push(pre);
        }

        let visited = &mut HashSet::<i32>::new();
        let cycle = &mut HashSet::<i32>::new();

        fn dfs(
            cur: i32,
            hm: &HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
            cycle: &mut HashSet<i32>,
        ) -> bool {
            if let Some(..) = cycle.get(&cur) {
                return false;
            }
            if let Some(..) = visited.get(&cur) {
                return true;
            }

            cycle.insert(cur);
            if let Some(neis) = hm.get(&cur) {
                for nei in neis {
                    if dfs(*nei, hm, visited, cycle) == false {
                        return false;
                    }
                }
            }
            cycle.remove(&cur);
            visited.insert(cur);

            true
        }

        for i in 0..num_courses {
            if dfs(i, hm, visited, cycle) == false {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
#[test]
fn main() {}
