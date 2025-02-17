use crate::stack::array_stack::ArrayStack;

/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
/// 有效字符串需满足：
/// - 左括号必须用相同类型的右括号闭合。
/// - 左括号必须以正确的顺序闭合。
/// - 每个右括号都有一个对应的相同类型的左括号。
#[test]
fn test_leedcode_20() {
    pub fn is_valid(s: String) -> bool {
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

    assert!(is_valid(String::from("1")));
    assert!(is_valid(String::from(
        "{234{3[222]}111}()(([[23232323]])){}"
    )));
    assert_eq!(
        is_valid(String::from("{234{3[222]}111}()(([[23232323]])){}}")),
        false
    );

    assert_eq!(is_valid(String::from("(])")), false);
}
