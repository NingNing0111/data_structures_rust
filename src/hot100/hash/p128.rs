use std::{cmp, collections::HashSet};

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut max_len = 0;
        for &item in set.iter() {
            // 只从起始序列开始
            if !set.contains(&(item - 1)) {
                let mut cur_num = item;
                let mut cur_len = 1;
                while set.contains(&(cur_num + 1)) {
                    cur_num += 1;
                    cur_len += 1;
                }
                max_len = cmp::max(max_len, cur_len);
            }
        }
        max_len
    }
}

/// # Longest Consecutive Sequence
#[cfg(test)]
pub mod test_128 {
    use crate::hot100::hash::p128::Solution;

    #[test]
    fn test_p128() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        assert_eq!(
            Solution::longest_consecutive(vec![4, 3, 1, 2, 200, 45, 44]),
            4
        );
    }
}
