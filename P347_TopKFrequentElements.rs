impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Solution 1 -> sort and take top k -> O(n log n)

        // Solution 2 =>

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut count_s = vec![0i32; nums.len()];
    }
}
