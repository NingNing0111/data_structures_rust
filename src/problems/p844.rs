#[cfg(test)]
pub mod test {

    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_stack = Vec::<char>::new();
        let mut t_stack = Vec::<char>::new();
        for ch in s.chars() {
            if ch != '#' {
                s_stack.push(ch);
            } else {
                s_stack.pop();
            }
        }
        for ch in t.chars() {
            if ch != '#' {
                t_stack.push(ch);
            } else {
                t_stack.pop();
            }
        }
        return s_stack == t_stack;
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            backspace_compare("ab#c".to_string(), "ad#c".to_string()),
            true
        );
        assert_eq!(
            backspace_compare("ab##".to_string(), "c#d#".to_string()),
            true
        );
        assert_eq!(backspace_compare("a#c".to_string(), "a".to_string()), false);
    }
}
