use std::collections::HashSet;

struct Solution;

impl Solution {
    // pub fn length_of_longest_substring(s: String) -> i32 {
    //     let len = s.len();
    //     if len == 0 {
    //         return 0;
    //     }
    //     let (mut left, mut right, mut ans) = (0, 0, 0);
    //     let mut window = HashSet::<char>::new();
    //     let chars = s.chars().collect::<Vec<char>>();
    //     while right < len {
    //         if window.contains(&chars[right]) {
    //             window.remove(&chars[left]);
    //             left += 1;
    //         } else {
    //             window.insert(chars[right]);
    //             right += 1;
    //         }
    //         ans = ans.max(window.len());
    //     }
    //     ans as i32
    // }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let (mut left, mut right, mut max_len) = (0, 0, 0);
        let mut seen = [false; 128];
        while right < len {
            let c = bytes[right] as usize;
            if seen[c] {
                seen[bytes[left] as usize] = false;
                left += 1;
            } else {
                seen[c] = true;
                right += 1;
            }
            max_len = max_len.max(right - left);
        }
        max_len as i32
    }
}

/// # Longest Substring Without Repeating Characters
#[cfg(test)]
pub mod test_3 {
    use crate::hot100::sliding_window::p3::Solution;

    #[test]
    fn test_p3() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string())
        )
    }
}
