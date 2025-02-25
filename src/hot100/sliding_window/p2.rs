#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
            let mut ans = Vec::<i32>::new();
            if s.len() < p.len() {
                return ans;
            }
            let mut p_chars = p.chars().collect::<Vec<char>>();
            p_chars.sort();
            for i in 0..(s.len() - p.len() + 1) {
                let mut t_s = s[i..(i + p.len())]
                    .to_string()
                    .chars()
                    .collect::<Vec<char>>();
                t_s.sort();
                if p_chars == t_s {
                    ans.push(i as i32);
                }
            }
            ans
        }

        assert_eq!(
            find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );

        assert_eq!(
            find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );

        assert_eq!(
            find_anagrams("aaa".to_string(), "aaaaaaa".to_string()),
            vec![]
        );
    }
}
