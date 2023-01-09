/*
 242. Valid Anagram


Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Example 1:
Input: s = "anagram", t = "nagaram"
Output: true

*/
#[allow(unused)]
pub struct Solution {}

impl Solution {
    #[allow(unused)]
    pub fn is_anagram(s: String, t: String) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }
    #[test]
    fn test_2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
