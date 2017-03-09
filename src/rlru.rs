use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;

struct Rlru<K, V>
    where K: Eq + Hash
{
    cache: HashMap<K, Rc<RefCell<Node<V>>>>,
    head: Link<V>,
    tail: Link<V>,
    max_length: u32,
}


struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem: elem,
            prev: None,
            next: None,
        }
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;



impl<K, V> Rlru<K, V>
    where K: Eq + Hash
{
    pub fn new() -> Self {
        Rlru {
            cache: HashMap::new(),
            head: None,
            tail: None,
            max_length: 0,
        }
    }

    fn length_upkeep(&mut self) -> &mut Self {
        match self.cache.len() {
            len if len < self.max_length as usize => self,
            // placeholder. Will pop off LRU key.
            _ => self
        }
    }

    fn splice_node(&mut self, node: Rc<RefCell<Node<V>>>) {
        // update node's prev/next pointers to link to each other
        {
            let mut node = node.borrow_mut();

            // First we use cloning via Rc pointers, but will clean up our references via 'take' in the next_node routine
            node.prev.clone().map(|prev_node| {
                prev_node.borrow_mut().next = node.next.clone();
            });

            // here we take the extracted node's remaining pointers.
            node.next.take().map(|next_node| {
                next_node.borrow_mut().prev = node.prev.take();
            });
        }

        // insert node at head of rlru.
        self.push(node);
    }

    fn push(&mut self, node: Rc<RefCell<Node<V>>>) -> &mut Self {
        self.head.take().map(|prev_head| {
            prev_head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(prev_head);
        });
        self.head = Some(node.clone());
        self
    }

    fn pop(&mut self) {
        self.tail.take().map(|node| {
            if node.borrow().prev.is_none() {
                // have to worry about updating head.
                self.head.take();
            }
            self.tail = node.borrow_mut().prev.take();
        });
    }

    pub fn get<'a>(&'a mut self, key: K) -> Option<&'a V> {
        match self.cache.entry(key) {
            Entry::Occupied(entry) => {
                // splice node to front
                let found = entry.get();
                self.splice_node(found.clone());
                Some(&found.as_ref().borrow().elem)
            },
            Entry::Vacant(_) => None
        }
    }

    pub fn set(&mut self, key: K, elem: V) -> &mut Self {
        match self.cache.entry(key) {
            Entry::Occupied(node) => {
                // splice node to front & update value
                self
            },
            Entry::Vacant(_) =>  {
                // insert node to front & do ln upkeep
                self
            }
        }
    }
}
