type NodePtr<T> = Option<Box<Node<T>>>;
type NodeRef<'a, T> = Option<&'a Node<T>>;

pub struct Node<T> {
    elem: T,
    next: NodePtr<T>
}

pub struct List<T> {
    head: NodePtr<T>
}

pub struct Iter<'a, T> {
    ptr: NodeRef<'a, T>
}

pub struct IterMut<'a, T> {
    ptr: Option<&'a mut Node<T>>
}

pub struct IntoIter<T> {
    ptr: NodePtr<T>
}

impl<T> Node<T> {
    pub fn new(elem: T, next: NodePtr<T>) -> Self {
        Self {
            elem,
            next
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem, self.head.take());
        self.head.replace(Box::new(new_head));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.elem
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { ptr: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { ptr: self.head.as_deref_mut() }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { ptr: self.head }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.map(|node| {
            self.ptr = node.next.as_deref();
            &node.elem
        })
        
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.take().map(|node| {
            self.ptr = node.next;
            node.elem
        })
    }
}