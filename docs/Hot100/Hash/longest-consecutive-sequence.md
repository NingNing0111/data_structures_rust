# 最长连续子序列

## 题目描述

&emsp;在一个整数数组`nums`中，返回*最长的连续序列的长度*。例如`[3,2,4,1,200,45]`,最长连续序列长度为`[1,2,3,4]`,因此，返回 4。

## 思路

&emsp;为方便理解，可以将示例`[3,2,4,1,200,45]`,先排个序：`[1,2,3,4,45,200]`，显然最长连续序列的长度就是 4 了。一个策略就是：遍历数组 item，判断**item-1**在不在 nums 里，如果在，则当前 item 不是连续序列的起始元素，如果不在,则`item`就是一个连续序列的起始元素，此时，我们从`item`开始进行遍历，每次遍历都让$item+1$,当出现`item`不在 nums 里时，我们就得到了一个连续序列的长度$item_{last} - item_{start}$。我们维护一个 maxLen 遍历，每次结束，更新 maxLen 遍历即可。

&emsp;例如，对于原数组：[3,2,4,1,200,45]:

- 判断`item-1`是否在 nums 中：由于 3，2，4 元素都在，而 1 不在，因此 1 就是一个有序序列的起始元素，于是我们`item_{start} = 1`。
- 开始遍历并判断`item+1`是否在 nums 中，由于：2，3，4 都在 nums 中，5 不在，因此，得到一个连续序列的长度（5-1） = 4. 此时更新 maxLen。
- 继续再依次判断`200-1`和`45-1`是否在 nums 中；
- 最后返回 maxLen

&emsp;从算法流程上看，执行最多的操作就是判断 item 是否在 nums 中，对于一个查询操作来说，我们可以使用 HashMap 或 HashSet 实现 O(1)的查询操作。在本题中，使用 HashSet 即可。

## 代码

```rust
use std::{cmp, collections::HashSet};

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // 将nums转换为HashSet
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut max_len = 0;
        for &item in set.iter() {
            // 只从起始序列开始
            if !set.contains(&(item - 1)) {
                let mut cur_num = item;
                let mut cur_len = 1;
                // 开始求解连续序列的最大长度
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

```

### 代码分析

&emsp;明明两层循环，为什么时间复杂度是$O(n)$?我们对整个代码进行逐步分析：

- 首先`let set = nums.into_iter().collect::<HashSet<i32>>();`，将 nums 转为 HashSet，只需遍历 nums 一次，时间复杂度为$O(n)$。
- 对于`if !set.contains(&(item - 1))`,只有`num-1`不在 HashSet 里的时候，才从 num 开始查找连续序列，这样，每个数字只会作为序列起点最多一次，在查找`set.contains(&(cur_num + 1))`时，由于上一步操作已经排除了(item-1)之前的所有数，所以每个数进入内层 while 循环最多一次。因此，除去转换为 Set 那一步外，每个元素进入到 while 循环的次数最多一次，因此，总的时间复杂度为$O(n)$。
