> Problem: [1. 两数之和](https://leetcode.cn/problems/two-sum/description/)

[TOC]

# 思路

> 讲述看到这一题的思路

# 解题方法

> 描述你的解题方法

# 复杂度

时间复杂度:

> 添加时间复杂度, 示例： $O(n)$

空间复杂度:

> 添加空间复杂度, 示例： $O(n)$

# Code

```Rust []
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
```
