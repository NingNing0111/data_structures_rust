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
        let mut balance = true;
        let chs = ['(', ')', '[', ']', '{', '}'];
        for c in s.chars() {
            for i in 0..=2 {
                if c == chs[2 * i] {
                    array_stack.push(c);
                } else if c == chs[2 * i + 1] {
                    if array_stack.is_empty() {
                        balance = false; // 不平衡
                        break;
                    }
                    let peek_val = array_stack.peek().unwrap().clone();
                    if chs[2 * i] == peek_val {
                        array_stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        balance && array_stack.is_empty()
    }

    assert!(is_valid(String::from("")));
    assert!(is_valid(String::from("{{[]}}()(([[]])){}")));
    assert_eq!(is_valid(String::from("{{[]}}()(([[]])){}}")), false);
    assert_eq!(is_valid(String::from("(])")), false);
    assert!(is_valid(String::from("()")));
}

#[test]
fn test_loop() {
    for i in 1..=3 {
        println!("{}", i);
    }
}
