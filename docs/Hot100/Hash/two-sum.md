---
statistics: True
---

# 两数之和

## 题目描述

&emsp;给定一个数组`nums`和一个目标值`target`,找到一对下标`i`和`j`使得`nums[i] + nums[j] == target`。

## 思路

### 暴力求解

&emsp;先遍历$i(i=[0,n]$)，再遍历$j(j=[i+1,n])$,当满足条件`nums[i] + nums[j] == target`时，返回`[i,j]`即可。

&emsp;暴力求解的时间复杂度为：$O(n^2)$

```rust
#[cfg(test)]
pub mod test_1 {
    struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if nums[i] + nums[j] == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            return vec![-1, -1];
        }
    }

    #[test]
    fn test_p1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}

```

### Hash 优化

&emsp;观察暴力求解的实现，我们第二层的 for 循环是为了找到一个`j`使得`nums[i] + nums[j] == target`。转换可以得到`nums[j] = target-nums[i]`。

&emsp;也就是说，我们可以在遍历`nums[i]`的时候，使用一个 map 存储`nums[i]`的信息，其中 key 就是`nums[i]`,value 是`i`。当遍历到下一个`i`且满足`map.container(target-nums[i])`,我们所谓的`j`的下标就是`map[target-nums[i]]`。

&emsp;优化后的时间复杂度:$O(n)$，空间复杂度$O(n)$;

```rust
#[cfg(test)]
pub mod test_1 {
    use std::collections::HashMap;

    struct Solution;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut map = HashMap::<i32, i32>::new();
            for i in 0..nums.len() {
                if let Some(value) = map.get(&(target - nums[i])) {
                    return vec![i as i32, *value as i32];
                }
                map.insert(nums[i], i as i32);
            }
            return vec![-1, -1];
        }
    }

    #[test]
    fn test_p1() {
        assert_eq!(vec![1, 0], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}

```
