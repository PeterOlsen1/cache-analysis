///
/// This module contains all code that has to do with "RefList",
/// including the list and node implementations.
///
/// This list was created so that we can remove
/// from a list in O(1) time. This is achieved by keeping references
/// to nodes that we can instantly remove, instead of searching the list
///
pub mod list;
pub mod node;

pub use list::RefList;
pub use node::Node;
