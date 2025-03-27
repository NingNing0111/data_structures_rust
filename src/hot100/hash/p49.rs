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
