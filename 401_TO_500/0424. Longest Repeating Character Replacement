impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut letter = [0; 128];
        let s: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut max = 0;
        let mut max_freq = 0;

        for right in 0..s.len() {
            let ch = s[right] as usize;

            letter[ch] += 1;
            max_freq = max_freq.max(letter[ch]);

            if (right - left + 1) as i32 - max_freq > k {
                letter[s[left] as usize] -= 1;
                left += 1;
            }
            max = max.max(right - left + 1);
        }
        max as i32
    }
}
