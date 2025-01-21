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

pub struct Queue<T: Clone> {
    length: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        // Create new node with the given value
        let node = Rc::new(RefCell::new(Node::new(value)));
        self.length += 1;

        match self.tail.take() {
            // If we have a tail
            Some(tail) => {
                // Set tail's next to point to the new node (borrowing the mut with RefCell)
                // Tail is the last node of the queue, so we're just extending the recurrent "chain"
                tail.borrow_mut().next = Some(node.clone());
            }
            None => {
                // no tail == no head, so we're just adding the first node here
                self.head = Some(node.clone()) // #THERE
            }
        }

        // Setting queue's tail prop to point to the last added node
        // If we added first node then it will point to the head, (but with new, cause head cloned [created new] ref #THERE)
        self.tail = Some(node);
    }

    pub fn deque(&mut self) -> Option<T> {
        if self.head.is_none() {
            // No head -> return
            return None;
        }

        // We're accessing the head value through a map fn but we could just unwrap it from the Option. Works the same
        self.head.take().map(|old_head| {
            // Copying the next node of a header to then assign it to the queue.head in #HERE
            let next_node = old_head.borrow().next.clone();

            self.head = next_node; // #HERE

            if self.head.is_none() {
                // If head is empty -> reset tail
                self.tail = None;
            }

            self.length -= 1;

            // Returning the old head's value
            old_head.borrow().value.clone()
        })
    }
    pub fn peek(&self) -> Option<T> {
        // Another way of accessing the head value (this time to just return it's copied value
        match self.head.as_ref() {
            Some(head) => {
                // It sucks cause it's just too many operations and in the end I have to wrap it to Some again
                Some(head.borrow().value.clone())
            }
            None => None,
        }

        // This looks much better
        // self.head.as_ref().map(|head| head.borrow().value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn test_1() {
        let mut queue: Queue<u32> = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.peek(), Some(1));
        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(1));
        queue.enqueue(3);
        assert_eq!(queue.deque(), Some(1));
        assert_eq!(queue.peek(), Some(2));
        assert_eq!(queue.deque(), Some(2));
        queue.enqueue(4);
        assert_eq!(queue.peek(), Some(3));
        assert_eq!(queue.deque(), Some(3));
        assert_eq!(queue.peek(), Some(4));
        assert_eq!(queue.peek(), Some(4));
        assert_eq!(queue.peek(), Some(4));
        assert_eq!(queue.deque(), Some(4));
    }
}
