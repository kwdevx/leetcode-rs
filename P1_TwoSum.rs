impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(x) = map.get(v) {
                return vec![*x, i as i32];
            }

            map.insert(target - v, i as i32);
        }

        // base case
        return vec![0, 1];
    }
}
