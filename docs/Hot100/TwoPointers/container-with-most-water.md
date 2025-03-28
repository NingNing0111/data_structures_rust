---
statistics: True
---

# 盛最多水的容器

## 题目描述

&emsp;给定一个用于表示高度的数组`height`。同时有 n 条垂线，每条垂线的高度为`height[i]`,找到其中的两条线，使得它们与 `x` 轴共同构成的容器可以容纳最多的水。

&emsp;例如`height=[1,8,6,2,5,4,8,3,7]`,此时最多可以容纳`49`。

![alt text](images/container-with-most-water.png)

## 思路

&emsp;基于贪心+双指针。贪心策略体现在每次都要尝试寻找能盛水更多的值。首先，定义两个指针`i`和`j`，为了尽可能让容纳更大，两个指针的初始状态应该分别为：`i=0`和`j=len-1`。由于`短板效应`，`i`->`j`所能盛的水由`min(height[i],height[j])`决定，其值为`(j-i) * min(height[i],height[j])`。在下一次移动指针时，基于贪心策略，移动较小值的指针去尝试寻找更大的值。例如`height[i] < height[j]`,则让`i++`,尝试去找到一个`height[i'] > height[i]`，反之让`j--`。

## 代码

```rust
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

```
