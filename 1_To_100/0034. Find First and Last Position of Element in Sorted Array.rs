impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len = nums.len();
        let mut result: Vec<i32> = vec![-1, -1];
        let mut num1 = 0;
        let mut num2 = 0;
        for i in 0..len {
            if nums[i] == target && num1 == 0 {
                result[0] = i as i32;
                num1 += 1;
            }
            if nums[len - 1 - i] == target && num2 == 0 {
                result[1] = (len - 1 - i) as i32;
                num2 += 1;
            }
        }
        result
    }
}
