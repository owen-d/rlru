// #get
// #set
// #peek
// #new
// #clear
//has
use std::collections::HashMap;

// Doubly linked list
struct Node <V> {
    prev: Option<Box<Node<V>>>,
    next: Option<Box<Node<V>>>,
    val: V,
}

pub struct LRU <K, V> {
    cache: HashMap<K, V>,
    head: Option<Node<V>>,
    tail: Option<Node<V>>,
    len: u16,
}

impl <K: Eq + std::hash::Hash, V> LRU <K, V> {
    pub fn new() -> LRU<K, V> {
        LRU {
            cache: HashMap::new(),
            head: None,
            tail: None,
            len: 0
        }
    }
    pub fn set(&mut self, key: K, val: V) -> &mut LRU<K, V>{
        use std::collections::hash_map::Entry;

        match self.cache.entry(&key) {
            Entry::Occupied(_, val) => {},
            Entry::Vacant(_) => {}
        }
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
