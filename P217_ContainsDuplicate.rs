impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());

        for num in &nums {
            if !set.insert(num) {
                return true;
            }
        }

        return false;
    }
}
