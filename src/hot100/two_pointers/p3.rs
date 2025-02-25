#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
            let mut ans = Vec::<Vec<i32>>::new();
            let mut nums = nums.clone();
            nums.sort();
            let mut i = 0;
            while i < nums.len() {
                sorted_two_sum(nums.clone(), i, &mut ans);
                let t = nums[i];
                // 去重
                while i < nums.len() && nums[i] == t {
                    i += 1;
                }
            }
            ans
        }

        // 排序数组的 两数之和问题
        // 从start+1开始找到和为-nums[start]的所有组合
        fn sorted_two_sum(nums: Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            let mut l = start + 1;
            let mut r = nums.len() - 1;
            while l < r {
                if nums[l] + nums[r] + nums[start] < 0 {
                    l += 1;
                } else if nums[l] + nums[r] + nums[start] > 0 {
                    r -= 1;
                } else {
                    result.push(vec![nums[start], nums[l], nums[r]]);
                    // 去重
                    let t = nums[l];
                    while l < nums.len() && t == nums[l] {
                        l += 1;
                    }
                }
            }
        }

        let input = vec![-1, 0, 1, 2, -1, -4];
        let ans = three_sum(input);
        assert_eq!(ans, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }
}
