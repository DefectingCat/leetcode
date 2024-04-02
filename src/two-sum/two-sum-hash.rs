use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            let x = target - v;
            let y = table.get(&x);
            match y {
                Some(y) => return vec![i as i32, *y],
                None => {
                    table.insert(v, i as i32);
                }
            }
        }
        vec![]
    }
}
