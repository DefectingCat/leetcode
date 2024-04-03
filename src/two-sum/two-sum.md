> Problem: [1. 两数之和](https://leetcode.cn/problems/two-sum/description/)

[TOC]

# 思路

> 在一个数组中找到两数之和，该值为给定的数字，最后返回下标，且两个数不相同。最简单的思路就是在遍历数组的同时，
> 使用给定的数 `target` 减去当前的值，就会得到剩下相加所需要的值。再在数组内寻找是否有这个值就行了。

# 解题方法

> 遍历时，使用 `target` 减去当前值，之后直接在 Vec 中使用 `position` 寻找另一个值的下标。

# 复杂度

时间复杂度:

> 添加时间复杂度, 示例： $O(n)$

空间复杂度:

> 添加空间复杂度, 示例： $O(n)$

# Code

```Rust []
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
```
