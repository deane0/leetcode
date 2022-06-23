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
