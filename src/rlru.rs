use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;
// use std::cell::RefCell;

struct Rlru<K, V>
    where K: Eq + Hash
{
    cache: HashMap<K, Rc<Node<V>>>,
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

type Link<T> = Option<Rc<Node<T>>>;



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
        let max = self.max_length as usize;
        match self.cache.len() {
            0...max => self,
            // placeholder. Will pop off LRU key.
            _ => self
        }
    }

    fn splice_node(&mut self, node: Node<V>) {
        // update node's prev/next pointers to link to each other
        // insert node at head of rlru.
    }

    fn push(&mut self, node: Node<V>) -> &mut Self {
        let node = Rc::new(node);
        self.head.take().map(|prev_head| {
            prev_head.prev = Some(node.clone());
            node.next = Some(prev_head);
        });
        self.head = Some(node.clone());
        self
    }

    fn pop(&mut self) {
        self.tail.take().map(|node| {
            if node.prev.is_none() {
                // have to worry about updating head.
                self.head.take();
            }
            self.tail = node.prev;
        });
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        match self.cache.entry(key) {
            Entry::Occupied(node) => {
                // splice node to front
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
