use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sum_hash = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            if let Some(&test) = sum_hash.get(&(target - value)) {
                return vec![test as i32, index as i32]
            }
            sum_hash.insert(value, index);
        }
        vec![]
    }
}
