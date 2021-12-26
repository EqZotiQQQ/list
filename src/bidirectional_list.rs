// temporary for u32, later i will replace to generic

#![allow(unused_must_use)]
#![allow(dead_code)]
use std::fmt::{Debug, Display, Formatter};
use core::ptr::NonNull;
use crate::errors::Errors;

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Node {
    pub data: u32,
    pub next: Option<NonNull<Node>>,
    pub prev: Option<NonNull<Node>>,
}

impl Node {
    pub fn new(data: u32) -> Node {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BidirList {
    pub head: Option<NonNull<Node>>,
    pub tail: Option<NonNull<Node>>,
    pub len: u8,
}

impl BidirList {
    pub fn new() -> BidirList {
        BidirList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, new: u32) {
        let new_node:Box<Node> = Box::new(Node::new(new));
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

    pub fn pop_front(&mut self) -> Result<u32, Errors> {
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
    pub fn pop_back(&mut self) -> Result<u32, Errors> {
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

    pub fn push_back(&mut self, new: u32) {
        let new_node:Box<Node> = Box::new(Node::new(new));
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

impl BidirList {
    fn push_front_node(&mut self, mut node: Box<Node>) {
        node.next = self.head;
        let node = Some(Box::leak(node).into());
        unsafe {
            (*self.head.unwrap().as_ptr()).prev = node;
        }
        self.head = node;
    }

    fn push_back_node(&mut self, mut node: Box<Node>) {
        node.prev = self.tail;
        let node = Some(Box::leak(node).into());
        unsafe {
            (*self.tail.unwrap().as_ptr()).next = node;
        }
        self.tail = node;
    }

    fn add_first(&mut self, node: Box<Node>) {
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

impl Display for BidirList {
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

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.data)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // println!("Dropped node {}", self);
    }
}

impl Drop for BidirList {
    fn drop(&mut self) {
        // println!("Dropped List: {}", self);
        while !self.empty() {
            self.drop_front();
        }
    }
}
