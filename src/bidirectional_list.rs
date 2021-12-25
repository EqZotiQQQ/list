// temporary for u32, later i will replace to generic

use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use std::io::{BufRead, empty};
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
        let mut new_node:Box<Node> = Box::new(Node::new(new));
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

    pub fn pop_front(&mut self) -> Result<(), Errors> {
        if self.empty() {
            return Err(Errors::NoElementInListError);
        }
        Ok(())
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }
}

impl BidirList {
    fn push_front_node(&mut self, mut node: Box<Node>) {
        node.next = self.tail;
        let node = Some(Box::leak(node).into());
        unsafe {
            (*self.tail.unwrap().as_ptr()).prev = node;
        }
        self.tail = node;
    }

    fn add_first(&mut self, mut node: Box<Node>) {
        let node = Some(Box::leak(node).into());
        self.head = node;
        self.tail = node;
    }

    fn drop_front(&mut self) {
        let mut new_front: Option<NonNull<Node>> = None;
        unsafe {
            new_front = self.head.unwrap().as_ref().next;
        }
        std::mem::drop(self.head);
        self.head = new_front;
    }
}

impl Display for BidirList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut node = self.tail;
        write!(f, "[head|front]");
        while let Some(s) = node {
            write!(f, "<=>");
            unsafe {
                write!(f, "[{}]", s.as_ref().data);
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
        println!("Dropped {}", self.data);
    }
}

impl Drop for BidirList {
    fn drop(&mut self) {
        println!("Dropped {}", self);
        // todo: clear nodes
    }
}
