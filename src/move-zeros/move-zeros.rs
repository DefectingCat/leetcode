impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut left, mut right, len) = (0, 0, nums.len());
        while right < len {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
            right += 1;
        }
    }
}
