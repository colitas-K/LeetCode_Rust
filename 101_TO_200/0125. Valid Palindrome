impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars = s.chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .flat_map(|c| c.to_lowercase());

        while let (Some(l), Some(r)) = (chars.next(), chars.next_back()) {
            if l != r {
                return false;
            }
        }
        true
    }
}
