use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut set = Vec::new();
        let ss = s.split("").collect::<Vec<&str>>();
        let ss = &ss[1..ss.len() - 1];
        for v in ss {
            if set.contains(v) {
                if set.len() > res {
                    res = set.len();
                }
                let index = set.iter().position(|a| a == v).unwrap();
                set = set.split_at(index + 1).1.to_vec();
            }
            set.insert(set.len(), v);
        }
        if set.len() > res {
            res = set.len();
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string(),),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string(),),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string(),),
            3
        );
    }
}
