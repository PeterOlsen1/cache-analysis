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
}