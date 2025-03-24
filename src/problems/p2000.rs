#[cfg(test)]
pub mod test {

    ///
    /// abcdefd d
    /// dcba
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut stack = Vec::<char>::new();
        let mut ans = Vec::<char>::new();
        let mut find = false;
        for (_, c) in word.chars().enumerate() {
            if !find {
                stack.push(c);
                if c == ch {
                    find = true;
                }
            } else {
                while !stack.is_empty() {
                    ans.push(stack.pop().unwrap());
                }
                ans.push(c);
            }
        }
        if !find {
            return word.clone();
        } else {
            while !stack.is_empty() {
                ans.push(stack.pop().unwrap());
            }
        }

        return String::from_iter(ans);
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            reverse_prefix("abcdefd".to_string(), 'd'),
            "dcbaefd".to_string()
        );
        assert_eq!(reverse_prefix("d".to_string(), 'd'), "d".to_string());

        assert_eq!(
            reverse_prefix("abcdefd".to_string(), 'z'),
            "abcdefd".to_string()
        );
        assert_eq!(
            reverse_prefix("rzwuktxcjfpamlonbgyieqdvhs".to_string(), 's'),
            "shvdqeiygbnolmapfjcxtkuwzr".to_string()
        );
    }
}
