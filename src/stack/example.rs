/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
/// 有效字符串需满足：
/// - 左括号必须用相同类型的右括号闭合。
/// - 左括号必须以正确的顺序闭合。
/// - 每个右括号都有一个对应的相同类型的左括号。
#[test]
fn test_leedcode_20() {
    use crate::stack::simple_stack::ArrayStack;

    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }
        let mut array_stack: ArrayStack<char> = ArrayStack::new();
        for c in s.chars() {
            if c == '(' || c == '{' || c == '[' {
                array_stack.push(c);
            } else if c == ')' || c == '}' || c == ']' {
                if array_stack.is_empty() {
                    return false;
                }
                let peek_val = array_stack.peek().unwrap().clone();
                if c == ')' && peek_val == '('
                    || c == '}' && peek_val == '{'
                    || c == ']' && peek_val == '['
                {
                    array_stack.pop();
                } else {
                    return false;
                }
            }
        }
        array_stack.is_empty()
    }

    assert!(is_valid(String::from("")));
    assert!(is_valid(String::from("{{[]}}()(([[]])){}")));
    assert_eq!(is_valid(String::from("{{[]}}()(([[]])){}}")), false);

    assert_eq!(is_valid(String::from("(])")), false);
}
