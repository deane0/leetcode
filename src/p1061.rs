use crate::Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        assert_eq!(s1.len(), s2.len());

        let mut chars_s1 = s1.chars();
        let mut chars_s2 = s2.chars();
        let mut dict: Vec<Vec<char>> = vec![];

        while let (Some(char_s1), Some(char_s2)) = (chars_s1.next(), chars_s2.next()) {
            let mut is_first = true;
            let mut first_index = 0;
            let mut passed_index: usize = 0;
            let dict_clone = dict.clone();
            let mut dict_iter = dict_clone.iter();
            loop {
                if let Some(index) = dict_iter.position(|entry| {
                    passed_index += 1;
                    entry.contains(&char_s1) || entry.contains(&char_s2)
                }) {
                    if is_first {
                        dict[index].extend([char_s1, char_s2]);
                        dict[index].sort_unstable();
                        dict[index].dedup();

                        first_index = index;
                        is_first = false;
                    } else {
                        let index = passed_index - 1;
                        if !(dict[index].contains(&char_s1) && dict[index].contains(&char_s2)) {
                            let entry = dict.remove(index);
                            dict[first_index].extend(entry);
                            dict[first_index].sort_unstable();
                            dict[first_index].dedup();
                            passed_index -= 1;
                        };
                    }
                } else {
                    if is_first {
                        let mut entry = vec![char_s1, char_s2];
                        entry.sort_unstable();
                        entry.dedup();
                        dict.push(entry);
                    }
                    break;
                }
            }
        }

        base_str
            .chars()
            .map(|c| {
                dict.iter()
                    .find_map(|cc| if cc.contains(&c) { Some(cc[0]) } else { None })
                    .map_or(c, |c| c)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase1() {
        let result = Solution::smallest_equivalent_string(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string(),
        );

        assert_eq!(result, "makkek");
    }

    #[test]
    fn testcase2() {
        let result = Solution::smallest_equivalent_string(
            "hello".to_string(),
            "world".to_string(),
            "hold".to_string(),
        );

        assert_eq!(result, "hdld");
    }

    #[test]
    fn testcase3() {
        let result = Solution::smallest_equivalent_string(
            "leetcode".to_string(),
            "programs".to_string(),
            "sourcecode".to_string(),
        );

        assert_eq!(result, "aauaaaaada");
    }
}
