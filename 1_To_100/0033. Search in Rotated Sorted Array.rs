impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            if nums[left] <= nums[mid] {
                if nums[mid] == target {
                    return mid as i32;
                } else if nums[left] == target {
                    return left as i32;
                } else if nums[left] < target && nums[mid] > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] == target {
                    return mid as i32;
                } else if nums[right] == target {
                    return right as i32;
                } else if nums[right] > target && nums[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}
