use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::fmt::Display;

struct Rlru<K, V>
    where K: Eq + Hash + Clone
{
    cache: HashMap<K, Rc<RefCell<Node<K, V>>>>,
    head: Link<K, V>,
    tail: Link<K, V>,
    max_length: u64,
}

struct Node<K, V> {
    key: K,
    elem: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, elem: V) -> Self {
        Node {
            key: key,
            elem: elem,
            prev: None,
            next: None,
        }
    }
}

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;



impl<K, V> Rlru<K, V>
    where K: Display + Eq + Hash + Clone,
          V: Display
{
    pub fn new() -> Self {
        Rlru {
            cache: HashMap::new(),
            head: None,
            tail: None,
            max_length: 10,
        }
    }

    fn length_upkeep(&mut self) -> &mut Self {
        match self.cache.len() {
            len if len < self.max_length as usize => self,
            _ => {
                self.tail
                    .as_ref()
                    .map(|node| node.borrow().key.clone())
                    .and_then(|key| self.cache.remove(&key));
                self.pop();
                self
            }
        }
    }

    fn splice_node(&mut self, node: Rc<RefCell<Node<K, V>>>, push_front: bool) {
        // update node's prev/next pointers to link to each other
        {
            let mut node = node.borrow_mut();

            // First we use cloning via Rc pointers,
            // but will clean up our references via 'take' in the next_node routine
            node.prev.clone().map_or_else(
                // If this was the head of the list, we must update the head pointer to the new head
                || self.head = node.next.clone(),
                |prev_node| prev_node.borrow_mut().next = node.next.clone());

            // here we take the extracted node's remaining pointers.
            node.next.take().map(|next_node| next_node.borrow_mut().prev = node.prev.take());
        }

        // insert node at head of rlru.
        if push_front {
            self.push(node);
        }
    }

    fn push(&mut self, node: Rc<RefCell<Node<K, V>>>) -> &mut Self {
        self.head.take().map(|prev_head| {
            prev_head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(prev_head);
        });
        self.head = Some(node.clone());
        self.length_upkeep();
        self
    }

    fn pop(&mut self) -> Option<V> {
        self.tail.take().map(|node| {
            if node.borrow().prev.is_none() {
                // have to worry about updating head.
                self.head.take();
            }

            // Remove the ref to current node on the prev node
            node.borrow_mut().prev.as_mut().map(|new_prev| new_prev.borrow_mut().next.take());

            // Reassign self.tail ref & return elem
            self.tail = node.borrow_mut().prev.take();
            Rc::try_unwrap(node)
                .ok()
                .map(|node| node.into_inner().elem)
                .unwrap()
        })
    }

    fn pop_head(&mut self) -> Option<V> {
        self.head.take().map(|node| {
            if node.borrow().next.is_none() {
                // have to worry about updating tail.
                self.tail.take();
            }

            // Remove the ref to current node on the next node
            node.borrow_mut().next.as_mut().map(|new_next| new_next.borrow_mut().prev.take());

            //Reassign self.head ref & return elem
            self.head = node.borrow_mut().next.take();
            Rc::try_unwrap(node)
                .ok()
                .map(|node| node.into_inner().elem)
                .unwrap()
        })
    }

    pub fn get(&mut self, key: &K) -> Option<Ref<V>> {
        // Update node positions
        self.cache
            .get(key)
            .map(|node| node.clone())
            .map(|node| self.splice_node(node, true));

        // Return the ref.
        self.cache
            .get(key)
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn set(&mut self, key: K, elem: V) -> &mut Self {
        let new_node = Rc::new(RefCell::new(Node::new(key.clone(), elem)));

        self.cache
            .remove(&key)
            .map(|old_node| self.splice_node(old_node, false));

        self.push(new_node.clone());

        // update tail, assuming this is the only node.
        if self.tail.is_none() {
            self.tail = Some(new_node.clone());
        }

        self.cache.insert(key, new_node);
        self
    }

    // Iterator
    pub fn into_iter(mut self) -> IntoIter<K, V> {
        self.cache.clear();
        IntoIter(self)
    }
}

pub struct IntoIter<K: Display + Eq + Hash + Clone, V>(Rlru<K, V>);

impl<K: Display + Eq + Hash + Clone, V: Display> Iterator for IntoIter<K, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_head()
    }
}

mod test {
    use super::Rlru;
    use std::collections::HashMap;

    #[test]
    fn basics() {
        let mut lru = Rlru::new();

        lru.set("a", 1)
            .set("b", 2)
            .set("c", 3);

        assert_eq!(lru.tail.is_some(), true);

        lru.tail.take().map(|node| {
            assert_eq!(node.borrow().elem, 1);
            //replace
            lru.tail = Some(node);
        });


        let second = lru.get(&"b")
            .map(|x| *x);
        assert_eq!(second, Some(2));

        assert_eq!(lru.head.is_some(), true);

        lru.head.take().map(|node| {
            assert_eq!(node.borrow().elem, 2);
        });

    }

    #[test]
    fn into_iter() {
        let mut lru = Rlru::new();

        lru.set("a", 1)
            .set("b", 2)
            .set("c", 3);

        let iter = lru.into_iter();
        let innards = iter.collect::<Vec<_>>();
        assert_eq!(innards, [3, 2, 1]);
    }

    #[test]
    fn ordering() {
        let mut lru = Rlru::new();
        lru.set("a", 1)
            .set("b", 2)
            .set("c", 3)
            .set("b", 4);

        let innards = lru.into_iter().collect::<Vec<_>>();
        assert_eq!(innards, [4, 3, 1]);
    }

    #[test]
    fn limits() {
        let mut lru = Rlru {
            cache: HashMap::new(),
            head: None,
            tail: None,
            max_length: 2,
        };

        lru.set("a", 1)
            .set("b", 2)
            .set("c", 3);

        let innards = lru.into_iter().collect::<Vec<_>>();
        assert_eq!(innards, [3, 2]);
    }
}
