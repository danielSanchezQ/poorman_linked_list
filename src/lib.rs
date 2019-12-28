use std::rc::Rc;
use std::ops::{Index, Deref};
use std::process::Output;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    prev :  Option<Rc<RefCell<Node<T>>>>,
    next :  Option<Rc<RefCell<Node<T>>>>,
    value : Box<T>,
}

impl<T> Node<T> {
    pub fn get(&self) -> &T {
        self.value.as_ref()
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    first : Option<Rc<RefCell<Node<T>>>>,
    last  : Option<Rc<RefCell<Node<T>>>>,
    len : usize,
}


impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{
            first: None,
            last : None,
            len: 0
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn append(&mut self, val : T) {
        match self.last {
            None => {
                let node : Rc<RefCell<Node<T>>>= Rc::new(RefCell::new(
                    Node {
                    prev : None,
                    next : None,
                    value: Box::new(val),
                }));
                self.first = Some(node.clone());
                self.last = Some(node);
            }
            Some(ref last_node) => {
                let node : Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(
                    Node {
                    prev : Some(last_node.clone()),
                    next : None,
                    value: Box::new(val),
                }));
                let ln : &RefCell<Node<T>> = last_node.borrow();
                ln.borrow_mut().next = Some(node.clone());
                self.last = Some(node);
            }
        }
        self.len += 1;
    }

//    fn get(&self, i : usize) -> &T {
//        if i >= self.len() {
//            panic!()
//        }
//        let node : &RefCell<Node<T>> = self.last.as_ref().unwrap().borrow();
//        node.into_inner().get()
////        self.last.as_ref().unwrap()().get()
////        let mut node = self.first.as_ref().unwrap();
////        for _ in 0..i {
//////            node = &node.next.as_ref().unwrap();
////            node = &node.get().next.unwrap();
////        }
////        node.as_ref().borrow().value.clone()
//    }
}

impl<T> Index<usize> for LinkedList<T> where T : Clone {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a T {
        if i >= self.len() {
            panic!()
        }
        self.first.as_ref().unwrap().into_inner().get()
//        let mut node = self.first.as_ref().unwrap();

//        for _ in 0..i {
////            node = &node.next.as_ref().unwrap();
//            node = &*node.borrow().borrow().next.unwrap();
//        }
//        &*node.as_ref().borrow().value
    }
//    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
//        if i >= self.len() {
//            panic!()
//        }
//        let half = self.len()/2;
//        let mut node : Rc<RefCell<Node<T>>> = match i {
//            i if i <= half => {
//                self.first.as_ref().unwrap().clone()
//            }
//            _ => {
//                self.last.as_ref().unwrap().clone()
//            }
//        };
////        let next_f : fn(&Rc<RefCell<Node<T>>>) -> &Rc<RefCell<Node<T>>> = match i {
////            i if i <= half => {
////                | node : &Rc<RefCell<Node<T>>>| {
////                    let n : &RefCell<Node<T>> = node.borrow();
////                    n.borrow().next.as_ref().unwrap()
////                }
////            }
////            _ => {
////                | node : &Rc<RefCell<Node<T>>>| {
////                    let n : &RefCell<Node<T>> = node.borrow();
////                    n.borrow().prev.as_ref().unwrap()
////                }
////            }
////        };
//        let rng = match i {
//            i if i <= half => {
//                0..i
//            }
//            i => {
//                0..(self.len()-i)
//            }
//        };
//        for _ in rng {
//            node = match i {
//                i if i <= half => {
//                    *node.as_ref().borrow().next.as_ref().unwrap()
//                }
//                _ => {
//                    *node.as_ref().borrow().prev.as_ref().unwrap()
//                }
//            }
//        }
//        &node.as_ref().borrow().value
//    }
}


#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn empty() {
        let ll:LinkedList<i32> = LinkedList::new();
        assert_eq!(ll.len(), 0);
        assert!(ll.is_empty());
    }

    #[test]
    fn append() {
        let mut ll:LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            ll.append(i);
        }
        assert_eq!(ll.len(), 10);
    }

//    #[test]
//    fn access() {
//        let mut ll:LinkedList<i32> = LinkedList::new();
//        for i in 0..10 {
//            ll.append(i);
//        }
//        assert_eq!(ll.len(), 10);
//        assert_eq!(ll[0], 10);
//    }
}
