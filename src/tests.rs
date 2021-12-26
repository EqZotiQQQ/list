#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::bidirectional_list::{BidirList, Node};

    #[test]
    fn create_empty_list() {
        let head = None;
        let tail = None;

        let list = BidirList::new();

        assert_eq!(list.empty(), true);
        assert_eq!(list.head, head);
        assert_eq!(list.tail, tail);
    }

    #[test]
    fn push_to_empty_list() {
        let head = Some(Box::new(Node::new(42)));
        let tail = Some(Box::new(Node::new(42)));

        let mut list = BidirList::new();
        list.push_front(42);

        unsafe {
            assert_eq!(list.len, 1);
            assert_eq!(list.head.unwrap().as_ref().data, head.unwrap().as_ref().data);
            assert_eq!(list.tail.unwrap().as_ref().data, tail.unwrap().as_ref().data);
        }
    }

    #[test]
    fn push_to_non_empty_list() {
        let tail = Some(Rc::new(Node::new(42)));
        let head = Some(Rc::new(Node::new(24)));

        let mut list = BidirList::new();
        list.push_front(42);
        list.push_front(24);

        unsafe {
            assert_eq!(list.len, 2);
            assert_eq!(list.head.unwrap().as_ref().data, head.unwrap().as_ref().data);
            assert_eq!(list.tail.unwrap().as_ref().data, tail.unwrap().as_ref().data);
        }
    }

    #[test]
    fn test_display() { // Don't forget for "cargo test -- --nocapture"
        let mut list = BidirList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);
        list.push_front(40);
        list.push_front(50);

        println!("{}", list);
    }
}