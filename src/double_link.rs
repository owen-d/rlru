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

    fn push_head(&mut self, elem: T) -> &mut Self {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(prev_head) => {
                // set new_node's next prop to prev head
                new_node.borrow_mut().next = Some(prev_head);
                // set prev head's prev prop to new node
                new_node.borrow_mut().prev = Some(new_node.clone());
                // set self's head to new node
                self.head = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
        self
    }

    fn push_tail(&mut self, elem: T) -> &mut Self {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(prev_tail) => {
                prev_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(prev_tail);
                self.tail = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
        self
    }
}
