// Original source:
// Author: Howei
// Source: https://github.com/howeih/rust_kmp/blob/master/src/kmp/mod.rs

pub struct KMP {
    pattern: Vec<char>,
    failure_function: Vec<usize>,
    pattern_length: usize
}

impl KMP {
    pub fn new(pattern: &str) -> KMP {
        let pattern: Vec<char> = pattern.chars().collect();
        let pattern_length = pattern.len();
        KMP {
            failure_function: KMP::find_failure_function(&pattern),
            pattern: pattern,
            pattern_length: pattern_length
        }
    }

    fn find_failure_function(pattern: &Vec<char>) -> Vec<usize>{
        let mut i = 1;
        let mut j = 0;
        let pattern_length = pattern.len();
        let end_i: usize = pattern_length;
        let mut failure_function = vec![0usize; pattern_length];
        while i < end_i {
            if pattern[i] == pattern[j] {
                failure_function[i] = j + 1;
                i = i + 1;
                j = j + 1;
            } else {
                if j == 0 {
                    failure_function[i] = 0;
                    i = i + 1;
                } else {
                    j = failure_function[j - 1];
                }
            }
        }
        failure_function
    }

    pub fn count_overlap(&self, target: &str) -> i32 {
        let target: Vec<char> = target.chars().collect();
        let mut t_i: usize = 0;
        let mut p_i: usize = 0;
        let target_len = target.len();
        let mut result_idx = -1i32;
        let pattern_len = self.pattern_length;
        if self.pattern_length == 0 {
            return 0;
        }
        let mut count = 0;
        while t_i < target_len {
            if target[t_i] == self.pattern[p_i] {
                if result_idx == -1 {
                    result_idx = t_i as i32;
                }
                t_i = t_i + 1;
                p_i = p_i + 1;
                if p_i >= pattern_len{
                    count += 1;
                    p_i = self.failure_function[p_i - 1];
                }
            } else {
                if p_i == 0 {
                    p_i = 0;
                    t_i = t_i + 1;
                } else {
                    p_i = self.failure_function[p_i - 1];
                }
                result_idx = -1;
            }
        }
        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let kmp = KMP::new("abc");
        assert_eq!(kmp.count_overlap("123abc456"), 1);
    }

    #[test]
    fn test_no_match() {
        let kmp = KMP::new("abc");
        assert_eq!(kmp.count_overlap("123ab456"), 0);
    }

    #[test]
    fn test_multiple_non_overlapping_matches() {
        let kmp = KMP::new("abc");
        assert_eq!(kmp.count_overlap("abc123abc456abc"), 3);
    }

    #[test]
    fn test_multiple_overlapping_matches() {
        let kmp = KMP::new("aba");
        assert_eq!(kmp.count_overlap("abababa"), 3); // "abababa" -> "aba", "aba", "aba"
    }

    #[test]
    fn test_pattern_equals_target() {
        let kmp = KMP::new("abc");
        assert_eq!(kmp.count_overlap("abc"), 1);
    }

    #[test]
    fn test_pattern_longer_than_target() {
        let kmp = KMP::new("abcdef");
        assert_eq!(kmp.count_overlap("abc"), 0);
    }

    #[test]
    fn test_empty_pattern_and_target() {
        let kmp = KMP::new("");
        assert_eq!(kmp.count_overlap(""), 0); // Depending on interpretation, could be 0 or 1
    }

    #[test]
    fn test_empty_pattern() {
        let kmp = KMP::new("");
        assert_eq!(kmp.count_overlap("abc"), 0); // Depending on interpretation, could consider any string to contain the empty string once
    }

    #[test]
    fn test_empty_target() {
        let kmp = KMP::new("abc");
        assert_eq!(kmp.count_overlap(""), 0);
    }
}
