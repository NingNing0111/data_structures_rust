struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r, mut ans) = (0, height.len() - 1, 0);
        while l < r {
            let h = height[l].min(height[r]);
            ans = ans.max((r - l) as i32 * h);
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        ans
    }
}

/// # Container With Most Water
#[cfg(test)]
pub mod test_11 {
    use crate::hot100::two_pointers::p11::Solution;

    #[test]
    fn test_p11() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
