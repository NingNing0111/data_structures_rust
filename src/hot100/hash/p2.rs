#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    #[test]
    fn test() {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
            let mut map = HashMap::<Vec<u8>, Vec<String>>::new();
            for s in strs {
                let mut sorted_s = s.clone().into_bytes();
                sorted_s.sort_unstable();
                map.entry(sorted_s).or_insert(vec![]).push(s);
            }
            map.into_values().collect()
        }

        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut ans = group_anagrams(input);
        ans.sort();

        print!("{:?}", ans);
        assert_eq!(
            ans,
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        )
    }
}
