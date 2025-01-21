use std::cell::RefCell;
use std::rc::Rc;

type LinkedNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    value: T,
    next: Option<LinkedNode<T>>,
    prev: Option<LinkedNode<T>>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> LinkedNode<T> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }

    pub fn forward_traversal(&mut self) -> Option<&LinkedNode<T>> {
        match self.next.as_ref() {
            Some(node) => Some(node),
            None => None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<LinkedNode<T>>,
    tail: Option<LinkedNode<T>>,
    length: usize,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn insert_at_head(&mut self, value: T) {
        self.length += 1;

        let new_node = Node::new(value);

        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
            return;
        }

        new_node.borrow_mut().next = Some(Rc::clone(&self.head.as_ref().unwrap()));
        self.head.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&new_node));
        self.head = Some(Rc::clone(&new_node));
    }

    pub fn insert_at_tail(&mut self, value: T) {
        self.length += 1;

        let new_node = Node::new(value);

        if self.tail.is_none() {
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
            return;
        }

        new_node.borrow_mut().prev = Some(Rc::clone(&self.tail.as_ref().unwrap()));
        self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
        self.tail = Some(Rc::clone(&new_node));
    }

    pub fn peek_head(&self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        Some(self.head.as_ref().unwrap().borrow().value.clone())
    }

    pub fn peek_tail(&self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }

        Some(self.tail.as_ref().unwrap().borrow().value.clone())
    }

    pub fn insert_after(&mut self, pos: u32, value: T) {
        if self.length == 0 {
            panic!("DoublyLinked list is empty");
        }
        if pos >= self.length as u32 {
            panic!("Pos is out of bounds");
        }

        let new_node = Node::new(value);

        let mut current_node = Rc::clone(self.head.as_ref().unwrap());
        for _ in 0..pos {
            let next_node = {
                let current_node_ref = current_node.borrow(); // Create a scoped borrow
                Rc::clone(current_node_ref.next.as_ref().unwrap()) // Clone next node safely
            };
            current_node = next_node;
        }

        let next_node = current_node.borrow_mut().next.take();
        new_node.borrow_mut().prev = Some(Rc::clone(&current_node));
        new_node.borrow_mut().next = next_node.clone(); // clone possible None

        if let Some(next) = next_node {
            next.borrow_mut().prev = Some(Rc::clone(&new_node));
        } else {
            self.tail = Some(Rc::clone(&new_node));
        }

        current_node.borrow_mut().next = Some(new_node);

        self.length += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::doubly_linked_list::DoublyLinkedList;

    #[test]
    fn insert_at_head() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_at_head(1);
        assert_eq!(list.peek_head(), Some(1));
        assert_eq!(list.peek_tail(), Some(1));
        list.insert_at_head(2);
        assert_eq!(list.peek_head(), Some(2));
        assert_eq!(list.peek_tail(), Some(1));
        assert_eq!(list.length, 2);
        list.insert_at_head(3);
        list.insert_at_head(4);
        assert_eq!(list.length, 4);
        assert_eq!(list.peek_head(), Some(4));
        assert_eq!(list.peek_tail(), Some(1));
    }

    #[test]
    fn insert_at_tail() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_at_tail(1);
        assert_eq!(list.peek_head(), Some(1));
        assert_eq!(list.peek_tail(), Some(1));
        list.insert_at_tail(2);
        assert_eq!(list.peek_head(), Some(1));
        assert_eq!(list.peek_tail(), Some(2));
        assert_eq!(list.length, 2);
        list.insert_at_tail(3);
        list.insert_at_tail(4);
        assert_eq!(list.length, 4);
        assert_eq!(list.peek_head(), Some(1));
        assert_eq!(list.peek_tail(), Some(4));
    }

    #[test]
    fn insert_after_success() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        list.insert_at_tail(4);
        list.insert_after(1, 9);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            9
        );
    }

    #[test]
    #[should_panic(expected = "DoublyLinked list is empty")]
    fn insert_after_empty_list() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_after(0, 9)
    }

    #[test]
    #[should_panic(expected = "Pos is out of bounds")]
    fn insert_after_oob() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_at_head(1);
        list.insert_after(1, 9)
    }

    #[test]
    fn traverse_forward() {
        let mut list: DoublyLinkedList<u32> = DoublyLinkedList::new();
        list.insert_at_tail(1);
        list.insert_at_tail(2);
        list.insert_at_tail(3);
        assert_eq!(list.length, 3);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            2
        );
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            3
        );
    }
}
