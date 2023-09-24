use std::{fmt::Debug, ops::Deref};

#[derive(Debug, Clone)]
struct Node<T: Clone> {
    value: T,
    prev: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct DoublyLinkedList<T: Clone> {
    pub len: usize,
    head: Option<Node<T>>,
}

impl<T: Clone + Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList { len: 0, head: None }
    }

    pub fn insert_at(&mut self, item: T, index: usize) {
        todo!();
    }

    pub fn remove(&mut self, item: T) -> Option<T> {
        todo!();
    }

    pub fn remove_at(&mut self, index: usize) -> Option<T> {
        todo!();
    }

    pub fn append(&mut self, item: T) {
        todo!();
    }

    pub fn prepend(&mut self, item: T) {
        self.len += 1;

        let mut node = Node {
            value: item,
            prev: None,
            next: self.head.take().map(Box::new),
        };

        if node.next.is_some() {
            node.next.as_mut().unwrap().prev = Some(Box::new(node.clone()));
        }

        self.head = Some(node);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        todo!();
    }
}
