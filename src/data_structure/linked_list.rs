use std::marker::PhantomData;
use std::ptr::NonNull;

// Generic struct
struct Node<T> {
    val: T,
    // Option<>: prev and next ptr can be None or Some
    // NonNull<>: the value will not be non-null pointer
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>
}

// new() constructor implementation
impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None
        }
    }
}

/*
pub struct LinkedList<T> {
    length: u32,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            marker: PhantomData
        }
    }

    pub fn insert_to_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node))});
        match self.head {
            None => self.tail = node_ptr;
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).pre = node_ptr}
        }
        self.head = node_ptr;
        self.length += 1;
    }

    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.pre = self.tail;
        let node_ptr = Some(unsafe {NonNull::new_unchecked(Box::into_raw(node))});

        match self.tail {
            None => self.head = node_ptr;
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr}
        }
        self.tail = node_ptr;
        self.length += 1;
    }
}
*/