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
                node.borrow_mut().prev = Some(tail.clone());
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

impl<T: Clone> RefList<T> {
    pub fn remove_node(&mut self, node: &Node<T>) -> T {
        // let data = &node.data;

        let prev = node.prev.clone();
        let next = node.next.clone();

        if let Some(prev_node) = prev.as_ref() {
            prev_node.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next_node) = next.as_ref() {
            next_node.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }

        self.len -= 1;
        node.data.to_owned()
    }

    pub fn pop_front(&mut self) -> Option<T> {        
        if self.head.is_none() {
            return None;
        }   

        self.len -= 1;
        self.head = self.head.clone().unwrap().borrow().next.clone();
        Some(self.head.as_ref().unwrap().borrow().data.clone())
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

    pub fn to_string(&self) -> String {
        let mut cur = self.head.clone();
        let mut out = String::new();

        while let Some(node) = cur {
            let val = &node.borrow().data;
            out.push_str(&format!("{:?}, ", val));
            cur = node.borrow().next.clone();
        };

        if out.len() >= 2 {
            out.truncate(out.len() - 2);
        }
        out
    }
}
