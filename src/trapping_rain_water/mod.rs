use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let (mut x, mut y) = (0, 1);

        let mut res = 0;
        let mut temp_res = 0;
        while x < height.len() - 2 {
            let x_value = height[x];
            if x_value == 0 {
                x += 1;
                y = x + 1;
                continue;
            }
            let y_value = height[y];

            match x_value {
                x_value if x_value > y_value && y == height.len() - 1 => {
                    x += 1;
                    y = x + 1;
                    temp_res = 0;
                    continue;
                }
                x_value if x_value > y_value => {
                    let v = x_value - y_value;
                    temp_res += v;
                    y += 1;
                    dbg!("1", x_value, y_value, temp_res, res);
                }
                x_value if x_value <= y_value => {
                    x = y;
                    y = x + 1;
                    res += temp_res;
                    temp_res = 0;
                    dbg!("2", x_value, y_value, temp_res, res);
                }
                _ => {
                    // dbg!("test");
                }
            }
        }
        res
    }
}
