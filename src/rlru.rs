use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::{RefCell, Ref};

struct Rlru<K, V>
    where K: Eq + Hash + Clone
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
    where K: Eq + Hash + Clone
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

    fn splice_node(&mut self, node: Rc<RefCell<Node<V>>>, push_front: bool) {
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
        if push_front {
            self.push(node);
        }
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

    pub fn get(&mut self, key: &K) -> Option<Ref<V>> {
        // Update node positions
        self.cache.get(key)
            .map(|node| node.clone())
            .map(|node| self.splice_node(node, true));

        // Return the ref.
        self.cache.get(key)
            .map(|node| {
                Ref::map(node.borrow(), |node| &node.elem)
            })
    }

    pub fn set(&mut self, key: K, elem: V) -> &mut Self {
        let mut new_node = Node::new(elem);

        self.cache.remove(&key)
            .map(|old_node| {
                self.splice_node(old_node, false);
            });

        new_node.next = self.head.take();
        let new_node = Rc::new(RefCell::new(new_node));
        self.head = Some(new_node.clone());
        self.cache.insert(key, new_node);
        self
    }
}
