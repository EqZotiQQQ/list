use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Node<T>
    where T: Display + Copy {
    pub data: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T>
    where T: Display + Copy {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            prev: None,
            next: None,
        }
    }
}

impl<T> Display for Node<T>
    where T: Display + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.data)
    }
}

impl<T> Drop for Node<T>
    where T: Display + Copy {
    fn drop(&mut self) {
        // println!("Dropped node {}", self);
    }
}
