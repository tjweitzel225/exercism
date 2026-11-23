use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

/// Invariant 1: All non-null pointers refer to live nodes owned by this list.
///              The list will deallocate these nodes when it is dropped.
/// Invariant 2: Pointers are consistently bidrectional.
///              If a.right == b, then b.left == a
/// Invariant 3: front.left is always null, back.right is always null
/// Invariant 4: front and back are null if and only if len == 0.
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    element: T,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    node: Link<T>,
}
pub struct Iter<'a, T> {
    curr: Link<T>,
    _phantom: std::marker::PhantomData<&'a T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let front = self.front;
        Cursor {
            list: self,
            node: front,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let back = self.back;
        Cursor {
            list: self,
            node: back,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            curr: self.front,
            _phantom: Default::default(),
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.node
            .map(|node| unsafe { &mut (*node.as_ptr()).element })
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        let n = self.deref(self.node?).next;
        self.node = n;
        n.map(|n| &mut self.deref(n).element)
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        let prev_node = self.deref(self.node?).prev;
        prev_node.map(|p| {
            self.node = prev_node;
            &mut self.deref(p).element
        })
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        // SAFETY: Pointer lifetime is tied to LinkedList's lifetime, which is borrowed with &mut.
        let node = unsafe { Box::from_raw(self.node?.as_ptr()) };
        self.list.len -= 1;
        match (node.prev, node.next) {
            (Some(p), Some(n)) => {
                self.deref(p).next = node.next;
                self.node = node.next;
                self.deref(n).prev = node.prev;
            }
            (Some(p), None) => {
                self.deref(p).next = node.next;
                self.node = node.prev;
                self.list.back = node.prev;
            }
            (None, Some(n)) => {
                self.deref(n).prev = node.prev;
                self.node = node.next;
                self.list.front = node.next;
            }
            (None, None) => {
                self.list.front = None;
                self.list.back = None;
                self.node = None;
            }
        }
        Some(node.element)
    }

    pub fn insert_after(&mut self, element: T) {
        self.list.len += 1;
        let new_node = Node {
            prev: None,
            next: None,
            element,
        };
        let new_ptr = NonNull::from_mut(Box::leak(Box::new(new_node)));
        match self.node {
            None => {
                let new = Some(new_ptr);
                self.list.front = new;
                self.list.back = new;
                self.node = new;
            }
            Some(a) => {
                self.deref(new_ptr).prev = Some(a);
                self.deref(new_ptr).next = self.deref(a).next;
                match self.deref(a).next {
                    None => self.list.back = Some(new_ptr),
                    Some(b) => self.deref(b).prev = Some(new_ptr),
                }
                self.deref(a).next = Some(new_ptr);
            }
        }
    }

    pub fn insert_before(&mut self, element: T) {
        self.list.len += 1;
        let new_node = Node {
            prev: None,
            next: None,
            element,
        };
        let new_ptr = NonNull::from_mut(Box::leak(Box::new(new_node)));
        match self.node {
            None => {
                let new = Some(new_ptr);
                self.list.front = new;
                self.list.back = new;
                self.node = new;
            }
            Some(a) => {
                self.deref(new_ptr).next = Some(a);
                self.deref(new_ptr).prev = self.deref(a).prev;
                match self.deref(a).prev {
                    None => self.list.front = Some(new_ptr),
                    Some(b) => self.deref(b).next = Some(new_ptr),
                }
                self.deref(a).prev = Some(new_ptr);
            }
        }
    }

    /// Get a mutable reference to the node the Cursor is on.
    /// SAFETY: The LinkedList maintains the node lifecycle, ensuring that node references remain valid.
    /// The Cursor's &mut borrow of the LinkedList ensures node references remain valid
    /// by tying dereferences to the Cursor/LinkedList lifetime.
    /// Null references are handled at the type-level via Option<NonNull<_>>
    fn deref(&mut self, n: NonNull<Node<T>>) -> &mut Node<T> {
        unsafe { &mut *n.as_ptr() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // SAFETY: We do not mutate, and only give out immutable references.
        let curr_node = unsafe { self.curr?.as_ref() };
        self.curr = curr_node.next;
        Some(&curr_node.element)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut c = self.cursor_front();
        while let Some(n) = c.take() {
            drop(n);
        }
    }
}
