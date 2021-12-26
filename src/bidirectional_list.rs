#![allow(unused_must_use)]
#![allow(dead_code)]
use std::fmt::{Debug, Display, Formatter};
use core::ptr::NonNull;
use crate::errors::Errors;
use crate::node::Node;


#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BidirList<T>
where T: Display + Copy {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    pub len: usize,
}

impl<T> BidirList<T>
where T: Display + Copy {
    pub fn new() -> BidirList<T> {
        BidirList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, new: T) {
        let new_node:Box<Node<T>> = Box::new(Node::new(new));
        match self.empty() {
            true => {
                self.add_first(new_node);
            }
            false => {
                self.push_front_node(new_node);
            }
        }

        self.len = self.len + 1;
    }

    pub fn pop_front(&mut self) -> Result<T, Errors> {
        if self.empty() {
            return Err(Errors::EmptyListError);
        }

        match self.head.map(|old_head| unsafe {
            let node = Box::from_raw(old_head.as_ptr());
            self.head = node.next;
            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.head = None,
            }
            self.len = self.len - 1;
            node.data
        }) {
            None => return Err(Errors::UnexpectedError),
            Some(result) => Ok(result),
        }
    }
    pub fn pop_back(&mut self) -> Result<T, Errors> {
        if self.empty() {
            return Err(Errors::EmptyListError);
        }

        match self.tail.map(|old_tail| unsafe {
            let node = Box::from_raw(old_tail.as_ptr());
            self.tail = node.prev;
            match self.tail {
                Some(tail) => (*tail.as_ptr()).next = None,
                None => self.tail = None,
            }
            self.len = self.len - 1;
            node.data
        }) {
            None => return Err(Errors::UnexpectedError),
            Some(result) => Ok(result),
        }
    }

    pub fn push_back(&mut self, new: T) {
        let new_node:Box<Node<T>> = Box::new(Node::new(new));
        match self.empty() {
            true => {
                self.add_first(new_node);
            }
            false => {
                self.push_back_node(new_node);
            }
        }
        self.len = self.len + 1;
    }

    pub fn display(&self) {
        let mut node = self.head;
        print!("List: [head|front]");
        while let Some(s) = node {
            print!("<=>");
            unsafe {
                print!("{}", s.as_ref());
                node = node.unwrap().as_ref().next;
            }
        }
        println!("<=>[tail|back]");
    }

    pub fn display_reversed(&self) {
        let mut node = self.tail;
        print!("Reversed list: [tail|back]");
        while let Some(s) = node {
            print!("<=>");
            unsafe {
                print!("{}", s.as_ref());
                node = node.unwrap().as_ref().prev;
            }
        }
        println!("<=>[head|front]");
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> BidirList<T>
where T: Display + Copy {
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;
        let node = Some(Box::leak(node).into());
        unsafe {
            (*self.head.unwrap().as_ptr()).prev = node;
        }
        self.head = node;
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        node.prev = self.tail;
        let node = Some(Box::leak(node).into());
        unsafe {
            (*self.tail.unwrap().as_ptr()).next = node;
        }
        self.tail = node;
    }

    fn add_first(&mut self, node: Box<Node<T>>) {
        let node = Some(Box::leak(node).into());
        self.head = node;
        self.tail = node;
    }

    fn drop_front(&mut self) {
        self.head.map(|old_head| unsafe {
            let node = Box::from_raw(old_head.as_ptr());
            self.head = node.next;
            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.head = None,
            }
            self.len = self.len - 1;
        });
    }
}

impl<T> Display for BidirList<T>
where T: Display + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut node = self.head;
        write!(f, "[head|front]");
        while let Some(s) = node {
            write!(f, "<=>");
            unsafe {
                write!(f, "{}", s.as_ref());
                node = node.unwrap().as_ref().next;
            }
        }
        write!(f, "<=>[tail|back]")
    }
}

impl<T> Drop for BidirList<T>
where T: Display + Copy {
    fn drop(&mut self) {
        // println!("Dropped List: {}", self);
        while !self.empty() {
            self.drop_front();
        }
    }
}
