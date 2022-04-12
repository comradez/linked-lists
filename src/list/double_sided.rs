use std::{ptr::null_mut, fmt::Display};

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>
}

pub struct Node<T> {
    data: T,
    prev: *mut Node<T>,
    next: *mut Node<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: null_mut(),
            tail: null_mut()
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Box::into_raw(Box::new(Node {
            data: elem,
            prev: null_mut(),
            next: null_mut()
        }));
        unsafe {
            if self.head.is_null() {
                self.tail = new_head;
            } else {
                (*self.head).prev = new_head;
                (*new_head).next = self.head;
            }
        }
        self.head = new_head;    
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            data: elem,
            prev: null_mut(),
            next: null_mut()
        }));
        unsafe {
            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
                (*new_tail).prev = self.tail;
            }
        }
        self.tail = new_tail;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                None
            } else { 
                let old_tail = Box::from_raw(self.tail);
                self.tail = old_tail.prev;
                if self.tail.is_null() {
                    self.head = null_mut();
                } else {
                    (*self.tail).next = null_mut();
                }
                Some(old_tail.data)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let old_head = Box::from_raw(self.head);
                self.head = old_head.next;
                if self.head.is_null() {
                    self.tail = null_mut();
                } else {
                    (*self.head).next = null_mut();
                }
                Some(old_head.data)
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter {
                prev: None,
                next: self.head.as_ref()
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while !self.head.is_null() {
            self.pop_front();
        }
    }
}

pub struct Iter<'a, T> {
    prev: Option<&'a Node<T>>,
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.prev = Some(node);
                self.next = node.next.as_ref();
                &node.data
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            self.prev.take().map(|node| {
                self.prev = node.prev.as_ref();
                self.next = Some(node);
                &node.data
            })
        }
    }
}

pub struct IterMut<'a, T> {
    prev: Option<&'a mut Node<T>>,
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                self.prev = (*node.next).prev.as_mut();
                &mut node.data
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            self.prev.take().map(|node| {
                self.prev = node.prev.as_mut();
                self.next = (*node.prev).next.as_mut();
                &mut node.data
            })
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut list = Self::new();
        iter.into_iter().for_each(|elem|list.push_back(elem));
        list
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.iter();
        while let Some(node_ref) = iter.next() {
            write!(f, "{} ", node_ref)?;
        }
        Ok(())
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self {
            head: null_mut(),
            tail: null_mut()
        }
    }
}