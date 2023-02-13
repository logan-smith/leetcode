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
        let mut s_map = HashMap::new();
        let mut t_map = HashMap::new();

        for c in s.chars() {
            let val = s_map.get(&c);
            match val {
                Some(num) => {
                    s_map.insert(c, num + 1);
                }
                None => {
                    s_map.insert(c, 1);
                }
            }
        }

        for c in t.chars() {
            let val = t_map.get(&c);
            match val {
                Some(num) => {
                    t_map.insert(c, num + 1);
                }
                None => {
                    t_map.insert(c, 1);
                }
            }
        }

        s_map == t_map
    }
}

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
