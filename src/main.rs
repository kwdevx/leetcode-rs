pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1i32; nums.len()];

    let mut prev = nums[0];
    for (i, num) in nums.iter().enumerate().skip(1) {
        res[i] = prev * res[i - 1];
        prev = *num;
    }

    prev = nums[nums.len() - 1];
    let mut postfix = 1;
    for (i, num) in nums.iter().enumerate().rev().skip(1) {
        postfix = prev * postfix;
        res[i] *= postfix;
        prev = *num;
    }

    res
}

fn main() {
    println!("{:?}", product_except_self(vec![1, 2, 3, 4]));
}
