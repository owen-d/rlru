// #get
// #set
// #peek
// #new
// #clear
//has
use std::collections::HashMap;
use std::rc::Rc;

// Doubly linked list
struct Node <V> {
    prev: Option<Box<Node<V>>>,
    next: Option<Box<Node<V>>>,
    val: V,
}

pub struct LRU <K, V> {
    cache: HashMap<K, Node<V>>,
    head: Option<Node<V>>,
    tail: Option<Node<V>>,
    len: u16,
}

impl <K: Eq + std::hash::Hash + Copy, V> LRU <K, V> {
    pub fn new() -> Self {
        LRU {
            cache: HashMap::new(),
            head: None,
            tail: None,
            len: 0
        }
    }
    pub fn set(&mut self, key: K, val: V) -> &mut LRU<K, V>{
        use std::collections::hash_map::Entry;

        match self.cache.entry(key) {
            Entry::Occupied(node) => {},
            Entry::Vacant(_) => {
                let node = Node {
                    prev: self.head.take().map(|x| Box::new(x)),
                    next: None,
                    val: val
                };
                self.cache.insert(key, node);
                self.head = Some(node);
            }
        };

        self
    }
}


#[cfg(test)]
mod tests {
    use super::LRU;

    #[test]
    fn lru_instantiaion() {
        let lru: LRU<&str, u8> = LRU::new();
    }

    // #[test]
}
