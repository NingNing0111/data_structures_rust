struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
pub struct LinkedList<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 头插法
    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.val
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }
}
