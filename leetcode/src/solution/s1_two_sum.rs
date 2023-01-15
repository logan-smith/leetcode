/*
 1. Two Sum


Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

*/
#[allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    #[allow(unused)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<&i32, i32> = HashMap::with_capacity(nums.len());

        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            let x = hashmap.get(&diff);
            match x {
                Some(index) => {
                    let res_vec: Vec<i32> = Vec::from([*index, i as i32]);
                    return res_vec;
                }
                None => {
                    hashmap.insert(num, i as i32);
                }
            }
        }
        // unreachable by problem definition
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum([2, 7, 11, 15].to_vec(), 9), [0, 1]);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum([3, 2, 4].to_vec(), 6), [1, 2]);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum([3, 3].to_vec(), 6), [0, 1]);
    }
}
