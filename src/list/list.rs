use super::Node;
use std::cell::RefCell;
use std::rc::Rc;

pub struct RefList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Rc<RefCell<Node<T>>>>,
    pub len: usize,
}

impl<T> RefList<T> {
    pub fn new() -> Self {
        RefList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    ///
    /// Push an item onto the back of the list,
    /// returns the node to keep as a pointer
    ///
    pub fn push_back(&mut self, data: T) -> Rc<RefCell<Node<T>>> {
        let node = Node::new(data);

        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node.clone());
            }
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }

        self.len += 1;
        node
    }
}

impl<T: std::fmt::Debug> RefList<T> {
    pub fn print(&self) {
        let mut cur = self.head.clone();

        while let Some(node) = cur {
            let val = &node.borrow().data;
            println!("{:?}", val);
            cur = node.borrow().next.clone();
        }
    }
}
