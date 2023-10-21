impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut res = 1;
        let mut set = std::collections::HashSet::<i32>::new();

        for n in nums {
            set.insert(n);
        }

        for n in set.iter() {
            if let None = set.get(&(*n - 1)) {
                let mut i = 1;
                while let Some(_) = set.get(&(*n + i)) {
                    i += 1
                }

                res = std::cmp::max(res, i);
            }
        }

        res
    }
}
