use std::rc::Rc;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;


impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None
        }
    }

    fn append(&self, elem: T) -> Self {
        let new_node = Node {
            elem: elem,
            next: self.head.clone(),
        };
        List {
            head: Some(Rc::new(new_node)),
        }
    }

    fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|x| x.next.clone())
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node)
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|item| {
            self.next = item.next.as_ref().map(|node| {
                &**node
            });
            &item.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(list.head(), Some(&2));
    }
}


