use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest_hash: HashMap<char, i32> = HashMap::new();
        let mut max = 0;
        let mut left = 0;

        for (right, letter) in s.chars().enumerate() {
            if let Some(&index) = longest_hash.get(&letter) {
                left = left.max(index + 1);
            }
            longest_hash.insert(letter, right as i32);
            max = max.max(right as i32 - left + 1);
        }
        max
    }
}
