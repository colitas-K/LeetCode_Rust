impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut result = 0;
        let vec: Vec<char> = s.chars().collect();
        if vec.len() < 3 {
            return 0;
        }
        for i in 0..vec.len() - 2 {
            if vec[i] != vec[i + 1] && vec[i] != vec[i + 2] && vec[i + 1] != vec[i + 2] {
                result += 1;
            }
        }
        result
    }
}
