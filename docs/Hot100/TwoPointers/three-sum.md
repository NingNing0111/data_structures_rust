---
statistics: True
---

# 三数之和

## 题目描述

&emsp;给定一个数组`nums`,返回所有三元组`[nums[i],nums[j],nums[k]]`使得`i!=j`、`i!=k`、`j!=k`和`nums[i] + nums[j] + nums[k] == 0`。**需要保证三元组不重复**。

例如:`nums = [-1,0,1,2,-1,-4]`返回 `[[-1,-1,2],[-1,0,1]]`

## 思路

1. 将三数之和转换为两数之和问题。遍历$item->nums$,则`-item`就是两数之和问题里的 target。在实际判断时，只需要判断`nums[i] + nums[j] + nums[k] == 0`即可。
2. 利用排序简化去重处理。因为题目要求返回的是值而不是下标，因此我们可以对数组排序。当我们对 item 求解完两数之和问题后，若 nextItem == item,则跳过。在求解两数之和问题时，若 itemNextJ == itemJ,同理也可以跳过。这样就能实现去重。
3. 对于一个有序的数组求解两数之和问题时，使用双指针更便捷。让`j=start`，`k=len-1`，其中`start = i`。若：
   - nums[j] + nums[k] < target,说明左边小了，让 j++；
   - nums[j] + nums[k] > target,说明右边大了,让 k--;
   - nums[j] + nums[k] == target,记录 nums[i],nums[j],nums[k]

## 代码

```rust
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
```
