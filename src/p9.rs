use crate::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if let Ok(x_rev) = x
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
        {
            x == x_rev
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_palindrome(10))
    }
}
