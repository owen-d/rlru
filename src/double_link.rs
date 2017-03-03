use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

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
                // set prev head's prev prop to new node
                prev_head.borrow_mut().prev = Some(new_node.clone());
                // set new_node's next prop to prev head
                new_node.borrow_mut().next = Some(prev_head);
                // set self's head to new node
                self.head = Some(new_node);
            }
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
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        };
        self
    }

    fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // remove the reference to the previous head.
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // If we've consumed the end of the list, we need to update the tail property.
                    // head has already been set to none via the 'take' call earlier.
                    self.tail.take();
                }
            };
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn pop_tail(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            };
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }


    fn peek_head(&mut self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    fn peek_tail(&mut self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    fn peek_head_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    fn peek_tail_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for DoubleLink<T> {
    fn drop(&mut self) {
        while self.pop_head().is_some() {}
    }
}

pub struct IntoIter<T>(DoubleLink<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_head()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_tail()
    }
}


#[cfg(test)]
mod test {
    use super::DoubleLink;

    #[test]
    fn basics() {
        let mut list = DoubleLink::new();

        // Check empty list behaves right
        assert_eq!(list.pop_head(), None);

        // Populate list
        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_head(4);
        list.push_head(5);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn backwards() {
        let mut list = DoubleLink::new();

        // Populate list
        list.push_tail(1);
        list.push_tail(2);
        list.push_tail(3);

        // check tailwards removal
        assert_eq!(list.pop_tail(), Some(3));
        assert_eq!(list.pop_tail(), Some(2));

        // push after pops
        list.push_tail(4).push_tail(5);
        assert_eq!(list.pop_tail(), Some(5));
        assert_eq!(list.pop_tail(), Some(4));

        assert_eq!(list.pop_tail(), Some(1));
        assert_eq!(list.pop_tail(), None);
    }

    #[test]
    fn peek() {
        let mut list = DoubleLink::new();
        assert!(list.peek_head().is_none());
        assert!(list.peek_tail().is_none());
        assert!(list.peek_head_mut().is_none());
        assert!(list.peek_tail_mut().is_none());

        list.push_head(1);
        list.push_head(2);
        list.push_head(3);

        assert_eq!(&*list.peek_head().unwrap(), &3);
        assert_eq!(&mut *list.peek_head_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_tail().unwrap(), &1);
        assert_eq!(&mut *list.peek_tail_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = DoubleLink::new();
        list.push_head(1); list.push_head(2); list.push_head(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
}
}
