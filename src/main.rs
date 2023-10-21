use std::collections::{HashMap, HashSet};

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Solution 1 -> sort and take top k -> O(n log n)

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut count_s = vec![HashSet::<i32>::new(); nums.len()];

    let mut top_index: usize = 0;

    for num in nums {
        // if key exist, mut the vec!
        let x = map.get_mut(&num);
        match x {
            None => {
                map.insert(num, 0);
                count_s[0].insert(num);
            }
            Some(xx) => {
                let temp = *xx + 1;
                if temp as usize > top_index {
                    top_index = temp as usize;
                }

                // delete bucket set previous count
                let pre = *xx;
                count_s[pre as usize].remove(&num);

                // update count bucket
                count_s[temp as usize].insert(num);
                // update map value
                *xx = temp;
            }
        }
    }

    let mut res: Vec<i32> = vec![];

    for i in (0..top_index + 1).rev() {
        let temp: Vec<i32> = count_s[i].clone().into_iter().collect();

        for x in temp {
            if res.len() == k as usize {
                return res;
            } else {
                res.append(&mut vec![x])
            }
        }
    }

    res
}

fn main() {
    println!("{:?}", top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2));
}
