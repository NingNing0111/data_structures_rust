#[cfg(test)]
pub mod test {
    use std::cmp;

    #[test]
    fn test() {
        pub fn trap(height: Vec<i32>) -> i32 {
            let mut ans = 0;
            let mut l = 0;
            let mut r = height.len() - 1;
            let mut l_max = 0;
            let mut r_max = 0;
            while l < r {
                l_max = cmp::max(l_max, height[l]);
                r_max = cmp::max(r_max, height[r]);

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

        let input = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let ans = trap(input);
        assert_eq!(ans, 6);

                let input = vec![4,2,0,3,2,5];
        let ans = trap(input);
        assert_eq!(ans, 9);
    }
}
