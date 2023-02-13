/*
49. Group Anagrams


Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

*/
#[allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    #[allow(unused)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // hashmap <k: [sorted string], v: [strings from input]>
        let mut hashmap = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();
            hashmap.entry(key).or_insert(vec![]).push(s);
        }

        hashmap.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // vec comparison doesn't consider out of order vecs
    fn test_1() {
        assert_eq!(
            Solution::group_anagrams(
                [
                    "eat".to_string(),
                    "tea".to_string(),
                    "tan".to_string(),
                    "ate".to_string(),
                    "nat".to_string(),
                    "bat".to_string()
                ]
                .to_vec()
            ),
            [
                ["bat".to_string()].to_vec(),
                ["nat".to_string(), "tan".to_string()].to_vec(),
                ["ate".to_string(), "eat".to_string(), "tea".to_string()].to_vec()
            ]
            .to_vec()
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::group_anagrams(["".to_string()].to_vec()),
            [["".to_string()].to_vec()]
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            Solution::group_anagrams(["a".to_string()].to_vec()),
            [["a".to_string()].to_vec()]
        );
    }
}
