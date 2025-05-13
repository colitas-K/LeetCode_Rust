impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut num = 0;
        for index in 0..s.len().saturating_sub(2) {
            if s[index] != s[index + 1] && s[index + 1] != s[index + 2] && s[index] != s[index + 2] {
                num += 1;
            }
        }

        num
    }
}
