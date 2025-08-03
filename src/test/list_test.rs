#[cfg(test)]
pub mod list_test {
    use crate::list::RefList;

    #[test]
    fn test_push() {
        let mut list: RefList<u16> = RefList::new();
        list.push_back(0);
        let node = list.push_back(1);
        list.push_back(2);

        // should be 0 1 2
        list.print();

        node.borrow().remove_self(&mut list);

        //should be 0 2
        println!("Printing list again, should not contain 1");
        list.print();
    }
}
