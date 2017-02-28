use std::rc::Rc;
use std::cell::{RefCell};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct DoubleLink<T> {
    head: Link<T>,
    tail: Link<T>,
}

pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    elem: T,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            elem: elem,
        }))
    }
}

impl<T> DoubleLink<T> {
    fn new() -> Self {
        DoubleLink {
            head: None,
            tail: None,
        }
    }
}
