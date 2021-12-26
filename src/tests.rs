#[cfg(test)]
mod tests {
    #![allow(irrefutable_let_patterns)]
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
    fn push_front_to_empty_list() {
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
    fn push_front_to_non_empty_list() {
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
    fn pop_front_from_empty_list() {
        let mut list = BidirList::new();
        match list.pop_front() {
            Ok(_) => {panic!("Unexpected success!");}
            Err(_) => {}
        }
    }

    #[test]
    fn pop_front_from_non_empty_list() {
        let expected = vec![50,40,30,20,10];
        let mut list = BidirList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);
        list.push_front(40);
        list.push_front(50);

        let mut i = 0;
        while !list.empty() {
            if let item = list.pop_front() {
                if let Ok(item_res) = item {
                    assert_eq!(expected[i], item_res);
                }
            }
            i = i + 1;
        }
    }

    #[test]
    fn push_front_and_pop_back() {
        let expected = vec![50,40,30,20,10];
        let mut list = BidirList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);
        list.push_front(40);
        list.push_front(50);

        while !list.empty() {
            if let item = list.pop_back() {
                if let Ok(item_res) = item {
                    assert_eq!(expected[(list.len) as usize], item_res);
                }
            }
        }
    }

    #[test]
    fn push_and_pop_back() {
        let expected = vec![10, 20, 30, 40, 50];
        let mut list = BidirList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);
        list.push_back(40);
        list.push_back(50);

        while !list.empty() {
            if let item = list.pop_back() {
                if let Ok(item_res) = item {
                    assert_eq!(expected[(list.len) as usize], item_res);
                }
            }
        }
    }

    #[test]
    fn pop_back_from_empty_list() {
        let mut list = BidirList::new();
        match list.pop_back() {
            Ok(_) => {panic!("Unexpected success!");}
            Err(_) => {}
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

        println!("\n{}", list);
        list.display_reversed();
        list.display();
    }
}