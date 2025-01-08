impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_vec: Vec<char> = s.chars().collect();
        let s_len = s.len();
        let t_len = t.len();
        let mut left = 0;
        let mut min_len = s_len + 1;
        let mut discriminant = -1;
        let mut t_letters = vec![0; 128];
        let mut window_letters = vec![0; 128];
        let mut need_letter = 0;

        for letter in t.chars() {
            t_letters[letter as usize] += 1;
        }

        for (right, right_letter) in s.chars().enumerate() {
            let right_letter = right_letter as usize;
            window_letters[right_letter] += 1;
            if window_letters[right_letter] <= t_letters[right_letter] {
                need_letter += 1;
            }
            while need_letter == t_len {
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    discriminant = left as i32;
                }

                let left_letter = s_vec[left] as usize;
                if window_letters[left_letter] <= t_letters[left_letter] {
                    need_letter -= 1;
                }
                window_letters[left_letter] -= 1;
                left += 1;
            }
        }
        if discriminant < 0 {
            return String::new();
        }
        s[discriminant as usize..(discriminant as usize + min_len)].to_string()
    }
}
