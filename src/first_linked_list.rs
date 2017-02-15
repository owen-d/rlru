
// struct Node <V> {
//     next: Option<Box<Node<V>>>,
//     val: V,
// }

// struct LinkedList <V> {
//     size: u16,
//     head: Option<Node<V>>,

// }

// impl <V> LinkedList <V> {
//     pub fn new() -> Self {
//         LinkedList {
//             size: 0,
//             head: None,
//         }
//     }
//     pub fn push(&mut self, val: V) -> &mut Self {
//         let added_node = Node {
//             val: val,
//             next: self.head.take().map(|x| Box::new(x))
//         };

//         self.head = Some(added_node);
//         self
//     }
//     pub fn pop(&mut self) -> Option<V> {
//         //we need to pull out the 'next' value from the head node
//         self.head.take().map(|node| {
//             self.head = node.next.map(|x| *x);
//             node.val
//         })
//     }
//     pub fn peek(&self) -> Option<&V> {
//         self.head.as_ref().map(|x| &x.val)
//     }
//     pub fn peek_mut(&mut self) -> Option<&mut V> {
//         self.head.as_mut().map(|x| &mut x.val)
//     }

//     // Iterator
//     pub fn into_iter(self) -> IntoIter<V> {
//         IntoIter(self)
//     }
// }

// impl <V> Drop for LinkedList <V> {
//     fn drop(&mut self) {
//         let mut cur_node = self.head.take();
//         while let Some(mut node) = cur_node {
//             cur_node = node.next.take().map(|x| *x);
//         }
//     }
// }

// pub struct IntoIter<T>(LinkedList<T>);

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop()
//     }
// }

// // Break

// pub struct Iter<'a, T: 'a> {
//     next: Option<&'a Node<T>>,
// }

// impl<T> LinkedList<T> {
//     pub fn iter<'a>(&'a self) -> Iter<'a, T> {
//         Iter {
//             next: self.head.as_ref().map(|node| node)
//         }
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.as_ref().map(|node| &**node);
//             &node.val
//         })
//     }
// }

// // Iter Mut

// pub struct IterMut<'a, T: 'a> {
//     next: Option<'a mut Node<T>>,
// }

// impl<T> LinkedList<T> {
//     pub fn iter_mut(&self) -> IterMut<T> {
//         IterMut {
//             next: self.head.as_mut().map(|node| &mut *node)
//         }
//     }
// }

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = 'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next = node.next.map(|node| &mut **node);
//             &mut node.val
//         })
//     }
// }


// mod test {
//     use super::LinkedList;
//     #[test]
//     fn basics() {
//         let mut list = LinkedList::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), None);
//     }

//     #[test]
//     fn peek() {
//         let mut list = LinkedList::new();
//         assert_eq!(list.peek(), None);
//         assert_eq!(list.peek_mut(), None);
//         list.push(1); list.push(2); list.push(3);

//         assert_eq!(list.peek(), Some(&3));
//         assert_eq!(list.peek_mut(), Some(&mut 3));
//     }

//     #[test]
//     fn into_iter() {
//         let mut list = LinkedList::new();
//         list.push(1); list.push(2); list.push(3);

//         let mut iter = list.into_iter();
//         assert_eq!(iter.next(), Some(3));
//         assert_eq!(iter.next(), Some(2));
//         assert_eq!(iter.next(), Some(1));
//     }

//     #[test]
//     fn iter() {
//         let mut list = LinkedList::new();
//         list.push(1); list.push(2); list.push(3);

//         let mut iter = list.iter();
//         assert_eq!(iter.next(), Some(&3));
//         assert_eq!(iter.next(), Some(&2));
//         assert_eq!(iter.next(), Some(&1));
//     }
// }
