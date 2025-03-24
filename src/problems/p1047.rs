#[cfg(test)]
pub mod test {

    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::<char>::new();
        for (_, ch) in s.chars().enumerate() {
            if !stack.is_empty() && *(stack.get(stack.len() - 1).unwrap()) == ch {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        return stack.iter().collect();
    }

    #[test]
    fn test_p1() {
        assert_eq!(remove_duplicates("abbaca".to_string()), "ca".to_string());
    }
}
