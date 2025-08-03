use super::RefList;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    pub data: T,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

///
/// Nodes are typically wrapped in an Rc<RefCell<>>,
/// use `.borrow()` to use node methods
///
impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }

    pub fn remove_self(&self, list: &mut RefList<T>) -> T
    where
        T: Clone,
    {
        let data = self.data.clone();

        let prev = self.prev.clone();
        let next = self.next.clone();

        if let Some(prev_node) = prev.as_ref() {
            prev_node.borrow_mut().next = next.clone();
        } else {
            list.head = next.clone();
        }

        if let Some(next_node) = next.as_ref() {
            next_node.borrow_mut().prev = prev.clone();
        } else {
            list.tail = prev.clone();
        }

        list.len -= 1;
        data
    }
}
