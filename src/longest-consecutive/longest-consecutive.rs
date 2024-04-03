use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let paire = vec![true; nums.len()];
        let map = nums.iter().zip(paire).collect::<HashMap<&i32, bool>>();

        map.iter().fold(0, |mut prev, cur| {
            let less_one = map.get(&(*cur.0 - 1));
            less_one.or_else(|| {
                let mut current = *cur.0 + 1;
                let mut current_longest = 1;
                loop {
                    let t = map.get(&current);
                    if t.is_none() {
                        break;
                    }
                    current += 1;
                    current_longest += 1;
                }
                if prev < current_longest {
                    prev = current_longest;
                }
                None
            });
            prev
        })
    }
}
