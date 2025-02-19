pub struct ArrayStack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        ArrayStack {
            top: 0,
            data: Vec::new(),
        }
    }

    // 入栈
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }

    // 出栈
    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    // 查看栈顶元素
    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    // 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        0 == self.top
    }

    // 大小
    pub fn size(&self) -> usize {
        self.top
    }

    // 清空数据
    pub fn drain(&mut self) {
        self.top = 0;
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayStack;

    #[test]
    fn test_stack_functionality() {
        let mut array_stack: ArrayStack<usize> = ArrayStack::new();

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

        array_stack.drain();
        assert!(array_stack.is_empty());
        assert_eq!(array_stack.size(), 0);
        assert_eq!(array_stack.peek(), None);
    }
}
