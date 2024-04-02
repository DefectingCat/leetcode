impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, v) in nums.iter().enumerate() {
            let x = target - v;
            let y = nums.iter().position(|n| n == &x);
            let i = i as i32;
            match y {
                // 数组中同一个元素在答案里不能重复出现。
                Some(y) if (i != y as i32) => return [i, y as i32].to_vec(),
                _ => {}
            }
        }
        vec![]
    }
}
