struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut ans = Vec::<i32>::new();
        let s_bytes = s.into_bytes();
        let p_bytes = p.into_bytes();
        print!("{}", s_bytes.len());
        print!("{}", p_bytes.len());
        // a -> 97
        if s_bytes.len() < p_bytes.len() {
            return ans;
        }

        let mut s_cnt = [0; 30];
        let mut p_cnt = [0; 30];

        for i in p_bytes.iter().enumerate() {
            s_cnt[s_bytes[i.0] as usize - 97] += 1;
            p_cnt[*i.1 as usize - 97] += 1;
        }
        if s_cnt == p_cnt {
            ans.push(0);
        }
        for i in 1..=(s_bytes.len() - p_bytes.len()) {
            s_cnt[s_bytes[i - 1] as usize - 97] -= 1;
            s_cnt[s_bytes[i + p_bytes.len() - 1] as usize - 97] += 1;
            if s_cnt == p_cnt {
                ans.push(i as i32 + 1);
            }
        }
        ans
    }
}

/// # Find All Anagrams in a String
#[cfg(test)]
pub mod test_438 {
    use super::Solution;

    #[test]
    fn test_p438() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams("ababab".to_string(), "ab".to_string())
        );
        // You can use `assert_eq!(target,Solution::function(args))` to call the function
    }
}
