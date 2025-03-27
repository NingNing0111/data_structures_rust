struct Solution;

impl Solution {
    // 对于有序数组的两数之和，可以使用双指针
    fn two_sum(i: usize, nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        let (mut j, mut k) = (i + 1, nums.len() - 1);
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if sum < 0 {
                j += 1;
            } else if sum > 0 {
                k -= 1;
            } else {
                ans.push(vec![nums[i], nums[j], nums[k]]);
                let t = nums[j];
                while j + 1 < nums.len() && nums[j + 1] == t {
                    j += 1;
                }
                k -= 1;
                j += 1;
            }
        }
    }

    // 将三数问题拆解为两数之和
    // nums[i]  == - (nums[k] + nums[j])
    // 遍历item->nums, 则-item就是两数之和的target，实际判断时，只需要判断nums[i] + nums[j] + nums[k] == 0即可。
    // 为了避免重复 ，我们可以对数组排序，当对item求解两数之和问题完毕后，若nextItem == item，则跳过
    // 在求解两数之和问题时，若nextJ == j,同理也跳过
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        nums.sort_unstable();
        let mut i = 0;
        while i < nums.len() {
            Self::two_sum(i, &nums, &mut ans);
            let t = nums[i];
            while i < nums.len() && nums[i] == t {
                i += 1
            }
        }
        ans
    }
}

/// # 3Sum
#[cfg(test)]
pub mod test_15 {
    use crate::hot100::two_pointers::p15::Solution;

    #[test]
    fn test_p15() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        )
    }
}
