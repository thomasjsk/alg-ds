use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    value: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, prev: None }))
    }
}

pub struct Stack<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node::new(value);
        self.length += 1;

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let head = self.head.clone().unwrap();
        new_node.borrow_mut().prev = Some(head);
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        self.length -= 1;

        self.head.take().map(|old_head| {
            let next_head = old_head.borrow_mut().prev.clone();
            self.head = next_head;

            old_head.borrow().value.clone()
        })
    }

    pub fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::Stack;

    #[test]
    fn test_1() {
        let mut stack: Stack<u32> = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(1);
        assert_eq!(stack.peek(), Some(1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(2));
        stack.push(3);
        stack.push(4);
        assert_eq!(stack.peek(), Some(4));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(1));
        stack.push(2);
        assert_eq!(stack.peek(), Some(2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.pop(), None);
    }
}
