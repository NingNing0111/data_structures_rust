#[cfg(test)]
pub mod test {

    #[test]
    fn test() {
        pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
            let mut ans = Vec::<i32>::new();
            if s.len() < p.len() {
                return ans;
            }
            let mut s_cnt: [i32; 30] = [0; 30];
            let mut p_cnt: [i32; 30] = [0; 30];
            let s = s.chars().collect::<Vec<char>>();
            let p = p.chars().collect::<Vec<char>>();
            for i in 0..p.len() {
                s_cnt[s[i] as usize - 'a' as usize] += 1;
                p_cnt[p[i] as usize - 'a' as usize] += 1;
            }

            if s_cnt == p_cnt {
                ans.push(0);
            }

            for i in 0..(s.len() - p.len()) {
                s_cnt[s[i] as usize - 'a' as usize] -= 1;
                s_cnt[s[i + p.len()] as usize - 'a' as usize] += 1;

                if s_cnt == p_cnt {
                    ans.push(i as i32 + 1);
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
