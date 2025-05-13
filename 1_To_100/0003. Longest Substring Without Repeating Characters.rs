use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut substring: HashMap<char, usize> = HashMap::new();
        let mut left = 0;

        for (right,letter) in s.chars().enumerate() {
            if let Some(&index) = substring.get(&letter) {
                left = left.max(index + 1);
            }
            substring.insert(letter, right);
            max = max.max(right - left + 1);
        }

        max as i32
    }
}
