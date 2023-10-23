#![allow(dead_code)]

struct Solution {}
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map: HashMap<String, HashMap<String, f64>> = HashMap::new();
        let mut res = vec![];

        for (i, v) in equations.iter().enumerate() {
            map.entry((*v)[0].to_string())
                .and_modify(|hm| {
                    hm.insert((*v)[1].to_string(), values[i]);
                })
                .or_insert(HashMap::from([((*v)[1].to_string(), values[i])]));

            map.entry((*v)[1].to_string())
                .and_modify(|hm| {
                    hm.insert((*v)[0].to_string(), 1f64 / values[i]);
                })
                .or_insert(HashMap::from([((*v)[0].to_string(), 1f64 / values[i])]));
        }
        println!("map: {map:?}");

        for v in &queries {
            let start = (*v)[0].to_string();
            let target = (*v)[1].to_string();
            res.push(Self::bfs(&map, start, target));
        }

        // for v in &queries {
        //     let start = (*v)[0].to_string();
        //     let target = (*v)[1].to_string();
        //     let mut visited = HashSet::<String>::new();

        //     res.push(Self::dfs(&map, &mut visited, &start, &target, 1f64));
        // }

        res
    }

    fn bfs(map: &HashMap<String, HashMap<String, f64>>, start: String, target: String) -> f64 {
        let mut visited = HashSet::<String>::new();
        let mut q = VecDeque::<(f64, String)>::new();
        q.push_back((1f64, start));

        println!("q: {q:?}");

        while !q.is_empty() {
            match q.pop_front() {
                Some((acc, start)) => {
                    match visited.get(&start) {
                        Some(_) => continue,
                        _ => (),
                    }
                    visited.insert(start.clone());

                    match map.get(&start) {
                        Some(start_map) => {
                            for (temp_to, edge) in start_map {
                                let new_acc = acc * *edge;

                                println!("new_acc: {new_acc:?}");

                                if temp_to.clone() == target {
                                    return new_acc;
                                } else {
                                    q.push_back((new_acc, temp_to.clone()));
                                }
                            }
                        }
                        _ => println!("get none"),
                    }
                }
                _ => println!("pop front none"),
            }
        }

        // base case
        return -1f64;
    }

    fn dfs(
        map: &HashMap<String, HashMap<String, f64>>,
        visited: &mut HashSet<String>,
        start: &String,
        target: &String,
        acc: f64,
    ) -> f64 {
        if !visited.insert(start.clone()) {
            println!("visited");
            return -1f64;
        }

        match map.get(start) {
            Some(x) => {
                if x.len() == 0 as usize {
                    println!(" len == 0");
                    return -1f64;
                }

                for (temp_to, edge) in x {
                    let new_acc = acc * edge;
                    match *temp_to == *target {
                        true => return new_acc.clone(),
                        false => {
                            let res = Self::dfs(map, visited, temp_to, target, new_acc.clone());
                            if res != -1f64 {
                                return res;
                            }
                        }
                    }
                }
            }
            None => {
                println!("cannot find key");
                return -1f64;
            }
        }

        -1f64
    }
}

#[cfg(test)]
#[test]
fn main() {
    assert_eq!(
        Solution::calc_equation(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()]
            ],
            vec![2.0, 3.0],
            vec![
                vec!["a".to_string(), "c".to_string()],
                vec!["b".to_string(), "a".to_string()],
                vec!["a".to_string(), "e".to_string()],
                vec!["a".to_string(), "a".to_string()],
                vec!["x".to_string(), "x".to_string()]
            ],
        ),
        vec![6.0, 0.5, -1.0, 1.0, -1.0] // vec![6.0]
    )
}
