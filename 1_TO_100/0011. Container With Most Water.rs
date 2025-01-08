use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max = 0;

        while left != right {
            let container = (right - left) * cmp::min(height[left] as usize, height[right] as usize);
            
            max = max.max(container);
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max as i32
    }
}
