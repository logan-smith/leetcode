/*
 347. Top K Frequent Elements


Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
*/
#[allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
        let mut counts = HashMap::with_capacity(nums.len()); // <k: num, v: count>
        let mut top_vec: Vec<i32> = Vec::new();

        for num in nums {
            let count = counts
                .entry(num)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for (num, count) in counts.into_iter() {
            freq[count].push(num);
        }

        freq.into_iter().rev().flatten().take(k as usize).collect()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
