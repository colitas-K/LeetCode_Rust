impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1 ;
        let mut result_left = -1;
        let mut result_right = -1;

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                result_left = mid;
                right = mid - 1;
            }
        }

        let mut left = 0;
        let mut right = nums.len() as i32 - 1 ;

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                result_right = mid;
                left = mid + 1;
            }
        }

        vec![result_left, result_right]
    }
}
