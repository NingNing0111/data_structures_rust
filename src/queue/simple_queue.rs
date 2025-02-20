pub struct Queue<T> {
    cap: usize, // 队列长度
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    // 队列长度
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // 入队列
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space avaiable".to_string());
        }

        self.data.insert(0, val);
        Ok(())
    }

    // 出队列
    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    // 判空
    pub fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    pub fn peek(&self) -> Option<&T> {
        if Self::is_empty(&self) {
            return None;
        }
        self.data.get(Self::size(&self) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_functionality() {
        let mut queue: Queue<char> = Queue::new(5);
        assert!(queue.is_empty());
        let _ = queue.enqueue('h');
        assert_eq!(false, queue.is_empty());
        let _ = queue.enqueue('e');
        let _ = queue.enqueue('l');
        assert_eq!(3, queue.size());
        let _ = queue.enqueue('l');
        let _ = queue.enqueue('o');
        if let Err(info) = queue.enqueue('!') {
            println!("{}", info)
        }
        while !queue.is_empty() {
            let c = queue.dequeue().unwrap();
            print!("{}", c);
        }
    }
}
