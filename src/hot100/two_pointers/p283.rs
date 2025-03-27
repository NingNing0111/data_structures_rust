struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j: usize = 0; // 始终指向非0元素
        for i in 0..nums.len() {
            if nums[i] != 0 {
                (nums[i], nums[j]) = (nums[j], nums[i]);
                j += 1
            }
        }
    }
}

/// # Move Zeroes
#[cfg(test)]
pub mod test_283 {
    use super::Solution;

    #[test]
    fn test_p283() {
        let mut arr = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut arr);
        assert_eq!(vec![1, 3, 12, 0, 0], arr);
    }
}
