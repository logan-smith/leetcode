/*
 217. Contains Duplicate

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:
Input: nums = [1,2,3,1]
Output: true
*/
#[allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    #[allow(unused)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashmap = HashMap::with_capacity(nums.len());

        for num in nums.iter() {
            match hashmap.get(&num) {
                Some(_) => {
                    return true;
                }
                None => {
                    hashmap.insert(num, 0);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }
    #[test]
    fn test_2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }
    #[test]
    fn test_3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
