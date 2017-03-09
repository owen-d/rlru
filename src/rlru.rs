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
    elem: Option<Rc<RefCell<T>>>,
    prev: Link<T>,
    next: Link<T>,
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
        let max = self.max_length as usize;
        match self.cache.len() {
            0...max => &mut self,
            // placeholder. Will pop off LRU key.
            _ => &mut self
        }
    }

    fn splice_node(&mut self, new_node: Node<V>) {
        // update node's prev/next pointers to link to each other
        // insert node at head of rlru.
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
                &mut self;
            },
            Entry::Vacant(_) =>  {
                // insert node to front & do ln upkeep
                &mut self;
            }
        }
    }
}
