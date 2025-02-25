use crate::list::simple_linked_list::LinkedList;

pub struct LinkedStack<T> {
    top: usize,
    data: LinkedList<T>,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack {
            top: 0,
            data: LinkedList::new(),
        }
    }
    // 入栈
    pub fn push(&mut self, val: T) {
        self.data.push_front(val);
        self.top += 1;
    }
    // 出栈
    pub fn pop(&mut self) {
        if !Self::is_empty(&self) {
            self.data.pop_front();
            self.top -= 1;
        }
    }
    // 查看栈顶元素
    pub fn peek(&mut self) -> Option<&T> {
        if Self::is_empty(&self) {
            return None;
        }
        self.data.peek_front()
    }
    // 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        0 == self.top
    }
    // 栈大小
    pub fn size(&self) -> usize {
        self.top
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedStack;

    #[test]
    fn test_stack_functionality() {
        let mut array_stack: LinkedStack<usize> = LinkedStack::new();

        assert!(array_stack.is_empty());

        array_stack.push(10);
        array_stack.push(8);
        array_stack.push(9);

        assert_eq!(array_stack.is_empty(), false);
        assert_eq!(array_stack.size(), 3);
        assert_eq!(array_stack.peek(), Some(&9));

        array_stack.pop();
        assert_eq!(array_stack.size(), 2);
        assert_eq!(array_stack.peek(), Some(&8));

        array_stack.pop();
        array_stack.pop();
        assert!(array_stack.is_empty());
        assert_eq!(array_stack.size(), 0);
        assert_eq!(array_stack.peek(), None);
    }
}
