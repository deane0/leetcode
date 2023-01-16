use std::collections::HashMap;

use crate::Solution;

impl Solution {
    // TODO: why HashMap
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mm: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match mm.get(&(target - v)) {
                Some(&index) => {
                    return vec![index, i as i32];
                }
                None => {
                    mm.entry(*v).or_insert(i as i32);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
