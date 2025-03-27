# 字母异位词分组

## 题目描述

&emsp;将一组字符串根据`异位词`规则进行分租。**异位词：A 将源单词的所有字母重新排列得到字母 B，此时 A、B 就是一组异位词**。例如：`abt` 和 `bta` 和`tab` 是一组异位词。

## 思路

&emsp;根据`异位词`的定义可以确定：**同一组异位词，其字母排序后都是一致的**。例如`abt`、`bta`、`tab`按字母排序后都是`abt`。因此我们可以使用一个 Map 存储，其中 Key 为`某单词字母排序后的字符串`，Value 为`Vec<String>`，每个元素按单词排序后都是 key。返回结果时，取出 map 里的所有 value 返回即可。

## 代码

```rust
#[cfg(test)]
/// # Group Anagrams
pub mod test_49 {
    use std::collections::HashMap;

    struct Solution;
    impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map = HashMap::<Vec<u8>, Vec<String>>::new();
            for str in strs {
                let mut sorted = str.as_bytes().to_vec();
                sorted.sort_unstable();
                map.entry(sorted).or_default().push(str);
            }
            map.into_values().collect()
        }
    }

    #[test]
    fn test_p49() {
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut ans = Solution::group_anagrams(input);
        ans.sort();
        assert_eq!(
            ans,
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        )
    }
}

```

&emsp;代码部分解释：key 并不是存储的按字符排序的字符串，而是将 str 转换为字节数组，然后根据字节数组进行排序。因为**Rust 中 String 底层存储的就是 Vec\<u8\>,每个 u8 占一个字节，`as_bytes()`可以获取 String 的字节切片，再调用`to_vec()`后可以转变为可变的 Vec\<u8\>，而 char 占 4 个字节，因此按 bytes 排序比按 char 排序在性能上更优，可以节省 u8 转 char 所需要的转换开销和 4 倍的内存**。
