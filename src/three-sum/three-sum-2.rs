use std::collections::HashMap;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // i j k
        // x is i
        // y is j
        // z_index is k
        let (mut x, mut y) = (0, 1);
        let max_x = nums.len() - 2;
        let max_y = nums.len() - 1;
        let rest_table = nums
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect::<HashMap<_, usize>>();

        let mut res = HashMap::new();
        while x < max_x {
            let z_value = 0 - (nums[x] + nums[y]);
            let z_index = rest_table.get(&z_value);
            match z_index {
                Some(index) => {
                    let index = *index;
                    if x != index && y != index {
                        let mut target = vec![nums[x], nums[y], nums[index]];
                        target.sort();
                        let key = format!("{}{}{}", target[0], target[1], target[2]);
                        res.entry(key).or_insert(target);
                    }
                }
                None => {}
            }
            if y < max_y {
                y += 1;
            } else {
                x += 1;
                y = x + 1;
            }
        }
        res.into_values().collect()
    }
}
