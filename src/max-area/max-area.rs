use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let max_index = height.len() - 1;
        let (mut left, mut right) = (0, max_index);
        let mut area = 0;
        loop {
            let left_value = height[left];
            let right_value = height[right];
            let current_area = match left_value.cmp(&right_value) {
                Ordering::Less | Ordering::Equal => {
                    let current_area = left_value * (right - left) as i32;
                    if left < max_index {
                        left += 1
                    } else {
                        if right > 0 {
                            right -= 1
                        };
                    }
                    current_area
                }
                Ordering::Greater => {
                    let current_area = right_value * (right - left) as i32;
                    if right > 0 {
                        right -= 1
                    } else {
                        if left < max_index {
                            left += 1
                        }
                    }
                    current_area
                }
            };
            if area < current_area {
                area = current_area
            }
            if left == max_index && right == 0 {
                break;
            }
        }
        area
    }
}
