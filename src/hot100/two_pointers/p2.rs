#[cfg(test)]
pub mod test {
    use std::cmp;

    #[test]
    fn test() {
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut l: usize = 0;
            let mut r: usize = height.len() - 1;
            let mut ans = 0;
            while l < r {
                let cur_ans = cmp::min(height[l], height[r]) * (r as i32 - l as i32);
                ans = cmp::max(ans, cur_ans);
                if height[l] < height[r] {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
            ans
        }
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let ans = max_area(input);
        assert_eq!(ans, 49);
    }
}
