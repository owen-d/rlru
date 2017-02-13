// #get
// #set
// #peek
// #new
// #clear
//has

// Doubly linked list
// struct Node <V> {
//     prev: Option<Box<Node<V>>>,
//     next: Option<Box<Node<V>>>,
//     val: V,
// }

// pub struct LRU <K, V> {
//     cache: HashMap<K, Node<V>>,
//     head: Option<Node<V>>,
//     tail: Option<Node<V>>,
//     len: u16,
// }

// impl <K: Eq + std::hash::Hash + Copy, V> LRU <K, V> {
//     pub fn new() -> Self {
//         LRU {
//             cache: HashMap::new(),
//             head: None,
//             tail: None,
//             len: 0
//         }
//     }
//     pub fn set(&mut self, key: &K, val: V) -> &mut LRU<K, V>{
//         use std::collections::hash_map::Entry;

//         match self.cache.get_mut(K).take() {
//             None => None,
//             Some(node) => {
//                 let node = *node;
//                 self.head = 
//             }
//         }
//         self
//     }
// }

struct Node <V> {
    next: Option<Box<Node<V>>>,
    val: V,
}

struct LinkedList <V> {
    size: u16,
    head: Option<Node<V>>,

}

impl <V> LinkedList <V> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: None,
        }
    }
    pub fn push(&mut self, val: V) -> &mut Self {
        let added_node = Node {
            val: val,
            next: self.head.take().map(|x| Box::new(x))
        };

        self.head = Some(added_node);
        self
    }
    pub fn pop(&mut self) -> Option<V> {
        //we need to pull out the 'next' value from the head node
        self.head.take().map(|node| {
            self.head = node.next.map(|x| *x);
            node.val
        })
    }
    pub fn peek(&self) -> Option<&V> {
        self.head.as_ref().map(|x| &x.val)
    }
    pub fn peek_mut(&mut self) -> Option<&mut V> {
        self.head.as_mut().map(|x| &mut x.val)
    }
}

impl <V> Drop for LinkedList <V> {
    fn drop(&mut self) {
        let mut cur_node = self.head.take();
        while let Some(mut node) = cur_node {
            cur_node = node.next.take().map(|x| *x);
        }
    }
}

pub struct IntoIter<T>(LinkedList<T>);


mod test {
    use super::LinkedList;
    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
}
