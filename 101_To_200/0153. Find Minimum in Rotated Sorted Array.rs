impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as usize - 1;

        if nums[left] < nums[right] {
            return nums[left];
        }
        
        let mut min = nums[right];
        
        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] < min {
                min = nums[mid];
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        min
    }
}
