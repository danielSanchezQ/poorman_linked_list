use std::ops::{Index};

#[derive(Debug)]
struct Node<T> {
    next :  Option<Box<Node<T>>>,
    value : Box<T>,
}

impl<T> Node<T> {
    pub fn new(val : T, next : Option<Box<Node<T>>>) -> Self {
        Node {
            next,
            value : Box::new(val)
        }
    }
    pub fn get(&self) -> &T {
        self.value.as_ref()
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    first : Option<Box<Node<T>>>,
    len : usize,
}


impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{
            first: None,
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
        match self.first.as_mut() {
            None => {
                self.first = Some(Box::new(Node::new(val, None)));
            },
            _ => {
                let mut node = self.first.as_mut().unwrap();
                for _ in 0..self.len-1 {
                    node = node.next.as_mut().unwrap();
                }
                node.next = Some(Box::new(Node::new(val, None)));
            }
        }
        self.len += 1;
    }

    fn get(&self, i : usize) -> &T {
        if i >= self.len() {
            panic!()
        }
        let mut node = self.first.as_ref().unwrap();
        for _ in 0..i {
            node = node.next.as_ref().unwrap()
        }
        node.get()
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        self.get(i)
    }
}

struct LLIter<'a, T: 'a> {
    ll : &'a LinkedList<T>,
    index : usize,
}

impl<'a, T> Iterator for LLIter<'a, T> {
    // we will be counting with usize
    type Item = &'a T;

    // next() is the only required method
    fn next(&mut self) -> Option<&'a T> {
        // Increment our count. This is why we started at zero.
        self.index += 1;

        // Check to see if we've finished counting or not.
        if self.index <= self.ll.len() {
            Some(self.ll.get(self.index-1))
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LLIter<'a, T>;

    fn into_iter(mut self) -> LLIter<'a, T> {
        LLIter{
            ll  : self,
            index : 0,
        }
    }
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

    #[test]
    fn access() {
        let mut ll:LinkedList<usize> = LinkedList::new();
        for i in 0..10 as usize {
            ll.append(i);
            assert_eq!(ll[i], i);
        }
        assert_eq!(ll.len(), 10);
        for (i, e) in (0..10).zip(ll.into_iter()) {
            assert_eq!(i as usize, *e);
        }
    }
}
