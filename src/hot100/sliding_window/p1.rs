#[cfg(test)]
pub mod test {
    use std::{cmp, collections::HashSet};

    #[test]
    fn test() {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let len = s.len();
            if len <= 0 {
                return 0;
            }
            let mut ans = 1;
            let mut r = 0;
            let mut l = 0;
            let mut windows = HashSet::<char>::new();
            let s = s.chars().collect::<Vec<char>>();
            while r < len {
                while windows.contains(&s[r]) {
                    windows.remove(&s[l]);
                    l += 1;
                }
                windows.insert(s[r]);
                ans = cmp::max(ans, r as i32 - l as i32 + 1);
                r += 1;
            }
            ans
        }

        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }
}
