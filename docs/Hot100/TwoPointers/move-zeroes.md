# 移动零

## 题目描述

&emsp;给定一个整数数组`nums`，将`nums`里的所有`0`移动到数组的右边，同时需要保持非零元素的相对顺序不变。

&emsp;例如:`[0,1,0,4,12]` ->`[1,4,12,0,0]`

## 思路

&emsp;题目要求把`0`移动到右边，其实就是将非零元素移动到数组的左边。定义双指针`i`和`j`.其中 i 用于遍历所有 nums 中的所有元素，找到非 0 元素，j 用于指向非零序列尾部的插入位置。当`nums[i] != 0`时，交换`nums[i]`和`nums[j]`的值即可。

## 代码

```rust
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut p0 = 0; // 指向的是非零序列尾部插入位置
        let mut p1 = 0; // 总是指向非0的位置
        while p0 < nums.len() {
            if nums[p0] != 0 {
                let t = nums[p0];
                nums[p0] = nums[p1];
                nums[p1] = t;
                p1 += 1;
            }
            p0 += 1;
        }
    }
}

```

&emsp;使用 Rust 语言特性，还可以继续简化代码：

```rust
struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j: usize = 0; // 始终指向非零序列尾部的插入位置
        for i in 0..nums.len() {
            if nums[i] != 0 {
                (nums[i], nums[j]) = (nums[j], nums[i]);
                j += 1 // 插入位置右移
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

```
