use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    pub fn append(&mut self, value: T) {
        self.next = Some(Rc::new(RefCell::new(Node::new(value))));
    }
}

pub struct Queue<T> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        self.length += 1;

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
            }
            None => self.head = Some(node.clone()),
        }

        self.tail = Some(node);
    }

    pub fn deque(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        self.head.take().map(|old_head| {
            let old_value = Rc::try_unwrap(old_head)
                .ok()
                .expect("Should only be one strong reference to the node")
                .into_inner()
                .value;
            self.head = old_head.borrow().next.clone();

            if self.head.is_none() {
                self.tail = None;
            }

            old_value
        })
    }

    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|head| head.borrow().value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn dequeue_1() {
        let mut queue: Queue<u32> = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.deque(), Some(1));
        assert_eq!(queue.deque(), Some(2));
        // assert_eq!(queue.deque(), None);
    }

    // fn peek_1() {
    //     let mut queue: Queue<u32> = Queue::new();
    //     queue.enqueue(3);
    //     assert_eq!(queue.peek(), Some(3));
    //     queue.enqueue(4);
    //     assert_eq!(queue.peek(), Some(4));
    // }
}
