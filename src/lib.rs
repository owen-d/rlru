// #get
// #set
// #peek
// #new
// #clear
//has
use std::collections::HashMap;

struct Node <V> {
    prev: Box<Option<Node<V>>>,
    next: Box<Option<Node<V>>>,
    val: V,
}

pub struct LRU <K, V> {
    cache: HashMap<K, V>,
    head: Node<V>,
    tail: Node<V>,
    len: u16,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
