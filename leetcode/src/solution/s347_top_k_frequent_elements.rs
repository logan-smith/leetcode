/*
 347. Top K Frequent Elements


Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
*/
#[allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        let mut top_vec = Vec::new();

        for num in nums {
            let count = counts
                .entry(num)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for _ in 0..k {
            // sorting k times is inefficient
            let top = counts
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap()
                .0
                .clone();
            top_vec.push(top);
            counts.remove(&top);
        }

        top_vec
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
