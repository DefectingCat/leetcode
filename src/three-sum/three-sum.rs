use std::collections::HashMap;

fn two_sum(nums: &[i32], target: i32, cur: i32) -> Vec<Vec<i32>> {
    let mut table = HashMap::new();

    nums.iter().enumerate().fold(vec![], |mut prev, (i, v)| {
        let x = target - v;
        let y = table.get(&x);
        match y {
            /* Some(y) if i != *y && cur_index != i && cur_index != *y => {
                let mut res = vec![nums[i], nums[*y], cur];
                res.sort();
                prev.push(res)
            } */
            Some(y) => {
                let mut res = vec![nums[i], nums[*y], cur];
                res.sort();
                prev.push(res)
            }
            None => {
                table.insert(v, i);
            }
        }
        prev
    })
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.iter()
            .enumerate()
            .fold(HashMap::new(), |mut prev, (index, cur)| {
                let target = 0 - cur;
                /* let rest = nums
                .iter()
                .enumerate()
                .filter(|(i, n)| *i != index)
                .map(|(i, n)| *n)
                .collect::<Vec<_>>(); */
                let mut rest = nums.clone();
                rest.remove(index);
                let mut res = two_sum(&rest, target, *cur);
                if res.len() == 0 {
                    return prev;
                }
                res.into_iter().for_each(|res| {
                    prev.entry(res.iter().map(|n| n.to_string()).collect::<String>())
                        .or_insert(res);
                });
                prev
            })
            .into_values()
            .collect()
    }
}
