#[cfg(test)]
pub mod test {

    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = Vec::<char>::new();
        let mut ans = Vec::<char>::new();
        for (_, c) in s.chars().enumerate() {
            if c == ')' {
                stack.pop();
            }
            if !stack.is_empty() {
                ans.push(c);
            }
            if c == '(' {
                stack.push(c);
            }
        }
        let res: String = ans.iter().collect();
        return res;
    }

    #[test]
    fn test_p1() {
        assert_eq!(remove_outer_parentheses("()()".to_string()), "".to_string());
    }
}
