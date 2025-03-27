struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut l_max, mut r, mut r_max) =
            (0, height[0], height.len() - 1, height[height.len() - 1]);
        let mut ans = 0;
        while l < r {
            l_max = l_max.max(height[l]);
            r_max = r_max.max(height[r]);
            if height[l] < height[r] {
                ans += l_max - height[l];
                l += 1;
            } else {
                ans += r_max - height[r];
                r -= 1;
            }
        }
        ans
    }
}

/// # Trapping Rain Water
#[cfg(test)]
pub mod test_42 {
    use crate::hot100::two_pointers::p42::Solution;

    #[test]
    fn test_p42() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
