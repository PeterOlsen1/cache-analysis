#[cfg(test)]
pub mod list_test {
    use crate::list::RefList;

    #[test]
    fn test_remove_middle() {
        let mut list: RefList<u16> = RefList::new();
        list.push_back(0);
        let node = list.push_back(1);
        list.push_back(2);

        assert_eq!(list.to_string(), "0, 1, 2");
        assert_eq!(list.len, 3);
        list.remove_node(&node.borrow());
        assert_eq!(list.to_string(), "0, 2");
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_remove_end() {
        let mut list: RefList<u16> = RefList::new();
        list.push_back(0);
        list.push_back(1);
        let node = list.push_back(2);

        assert_eq!(list.to_string(), "0, 1, 2");
        assert_eq!(list.len, 3);
        list.remove_node(&node.borrow());
        assert_eq!(list.to_string(), "0, 1");
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_remove_start() {
        let mut list: RefList<u16> = RefList::new();
        let node = list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.to_string(), "0, 1, 2");
        assert_eq!(list.len, 3);
        list.remove_node(&node.borrow());
        assert_eq!(list.to_string(), "1, 2");
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_pop_front() {
        let mut list: RefList<u16> = RefList::new();
        list.push_back(0);
        list.push_back(1);
        list.push_back(2);

        assert_eq!(list.to_string(), "0, 1, 2");
        assert_eq!(list.len, 3);
        list.pop_front();
        assert_eq!(list.to_string(), "1, 2");
        assert_eq!(list.len, 2);
    }

    #[test]
    fn test_remove_middle_pop_front() {
        let mut list: RefList<u16> = RefList::new();
        list.push_back(0);
        let node = list.push_back(1);
        list.push_back(2);

        assert_eq!(list.to_string(), "0, 1, 2");
        assert_eq!(list.len, 3);
        list.remove_node(&node.borrow());
        assert_eq!(list.to_string(), "0, 2");
        assert_eq!(list.len, 2);
        list.pop_front();
        assert_eq!(list.to_string(), "2");
        assert_eq!(list.len, 1);
    }
}
