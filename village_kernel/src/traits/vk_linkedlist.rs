//###########################################################################
// vk_list.rs
// Declarations of the list with atomic pointers
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use core::ptr;
use core::iter::Iterator;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicPtr, Ordering};

// List node struct with atomic pointers
struct ListNode<T> {
    obj: T,
    prev: AtomicPtr<ListNode<T>>,
    next: AtomicPtr<ListNode<T>>,
}

// List template with atomic pointers
pub struct LinkedList<T> {
    head: AtomicPtr<ListNode<T>>,
    tail: AtomicPtr<ListNode<T>>,
    iter: AtomicPtr<ListNode<T>>,
    len: usize,
}

// Impl list
impl<T> LinkedList<T> {
    // New
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
            iter: AtomicPtr::new(ptr::null_mut()),
            len: 0,
        }
    }

    // List get len
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> LinkedList<T> {
    // Push node to list with atomic operations
    pub fn push(&mut self, obj: T) {
        let node = ListNode {
            obj,
            prev: AtomicPtr::new(ptr::null_mut()),
            next: AtomicPtr::new(ptr::null_mut()),
        };
        
        let node_ptr = Box::into_raw(Box::new(node));
        
        if self.head.load(Ordering::Relaxed).is_null() {
            self.head.store(node_ptr, Ordering::Release);
            self.tail.store(node_ptr, Ordering::Release);
        } else {
            let tail = self.tail.load(Ordering::Acquire);
            unsafe {
                (*tail).next.store(node_ptr, Ordering::Release);
                (*node_ptr).prev.store(tail, Ordering::Release);
                self.tail.store(node_ptr, Ordering::Release);
            }
        }
        
        self.len += 1;
    }

    // Delete node from list with atomic operations
    pub fn del(&mut self, obj: &T)
    where
        T: PartialEq,
    {
        let mut current = self.head.load(Ordering::Acquire);
        
        while !current.is_null() {
            unsafe {
                let node = &*current;
                let obj_eq = &node.obj == obj;

                if obj_eq {
                    let prev = node.prev.load(Ordering::Acquire);
                    let next = node.next.load(Ordering::Acquire);
                    
                    if !prev.is_null() {
                        (*prev).next.store(next, Ordering::Release);
                    } else {
                        self.head.store(next, Ordering::Release);
                    }
                    
                    if !next.is_null() {
                        (*next).prev.store(prev, Ordering::Release);
                    } else {
                        self.tail.store(prev, Ordering::Release);
                    }
                    
                    // Free the node
                    drop(Box::from_raw(current));
                    self.len -= 1;
                }
                
                current = node.next.load(Ordering::Acquire);
            }
        }
    }

    // Clear List with atomic operations
    pub fn clear(&mut self) {
        let mut current = self.head.load(Ordering::Acquire);
        
        while !current.is_null() {
            unsafe {
                let next = (*current).next.load(Ordering::Acquire);
                drop(Box::from_raw(current));
                current = next;
            }
        }
        
        self.head.store(ptr::null_mut(), Ordering::Release);
        self.tail.store(ptr::null_mut(), Ordering::Release);
        self.len = 0;
    }
}

// LinkedList self iterator with atomic pointers
impl<T> LinkedList<T> {
    // List begin node
    pub fn begin(&mut self) {
        self.iter.store(self.head.load(Ordering::Acquire), Ordering::Release);
    }

    // List next node
    pub fn next(&mut self) {
        if !self.iter.load(Ordering::Acquire).is_null() {
            unsafe {
                self.iter.store(
                    (*self.iter.load(Ordering::Acquire)).next.load(Ordering::Acquire), 
                    Ordering::Release
                );
            }
        }
    }

    // List prev node
    pub fn prev(&mut self) {
        if !self.iter.load(Ordering::Acquire).is_null() {
            unsafe {
                self.iter.store(
                    (*self.iter.load(Ordering::Acquire)).prev.load(Ordering::Acquire), 
                    Ordering::Release
                );
            }
        }
    }

    // List end node
    pub fn end(&mut self) {
        self.iter.store(self.tail.load(Ordering::Acquire), Ordering::Release);
    }

    // List is begin
    pub fn is_begin(&self) -> bool {
        self.iter.load(Ordering::Acquire).is_null()
    }

    // List is end
    pub fn is_end(&self) -> bool {
        self.iter.load(Ordering::Acquire).is_null()
    }

    // List is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // List item
    pub fn item(&self) -> Option<&mut T> {
        let iter_ptr = self.iter.load(Ordering::Acquire);
        if iter_ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut (*iter_ptr).obj) }
        }
    }
}

// Impl drop for linked list
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// Impl form iterator for linkedlist
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// List iterator with atomic pointers
pub struct ListIterator<'a, T> {
    _list: &'a LinkedList<T>,
    current: AtomicPtr<ListNode<T>>,
    marker: PhantomData<&'a mut T>,
}

// List reverse iterator with atomic pointers
pub struct ListReverseIterator<'a, T> {
    _list: &'a LinkedList<T>,
    current: AtomicPtr<ListNode<T>>,
    marker: PhantomData<&'a mut T>,
}

// Impl iterator for list iterator with atomic operations
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.load(Ordering::Acquire);
        if current.is_null() {
            None
        } else {
            unsafe {
                let obj = &mut (*current).obj;
                self.current.store(
                    (*current).next.load(Ordering::Acquire), 
                    Ordering::Release
                );
                Some(obj)
            }
        }
    }
}

// Impl iterator for list reverse iterator with atomic operations
impl<'a, T> Iterator for ListReverseIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.load(Ordering::Acquire);
        if current.is_null() {
            None
        } else {
            unsafe {
                let obj = &mut (*current).obj;
                self.current.store(
                    (*current).prev.load(Ordering::Acquire), 
                    Ordering::Release
                );
                Some(obj)
            }
        }
    }
}

impl<T> LinkedList<T> {  
    // Create iterator
    pub fn iter_mut(&mut self) -> ListIterator<'_, T> {
        ListIterator {
            _list: self,
            current: AtomicPtr::new(self.head.load(Ordering::Acquire)),
            marker: PhantomData,
        }
    }

    // Create reverse iterator
    pub fn rev_iter_mut(&mut self) -> ListReverseIterator<'_, T> {
        ListReverseIterator {
            _list: self,
            current: AtomicPtr::new(self.tail.load(Ordering::Acquire)),
            marker: PhantomData,
        }
    }
}

// Impl linked list with atomic operations
impl<T> LinkedList<T> {
    /// Retains only the elements specified by the predicate.
    /// In other words, remove all elements `e` for which `f(&e)` returns false.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut current = self.head.load(Ordering::Acquire);
        let mut new_len = 0;

        while !current.is_null() {
            unsafe {
                let next = (*current).next.load(Ordering::Acquire);
                let should_retain = f(&(*current).obj);

                if !should_retain {
                    // Remove the node from the list
                    let prev = (*current).prev.load(Ordering::Acquire);
                    let next_node = next;

                    if !prev.is_null() {
                        (*prev).next.store(next_node, Ordering::Release);
                    } else {
                        self.head.store(next_node, Ordering::Release);
                    }

                    if !next_node.is_null() {
                        (*next_node).prev.store(prev, Ordering::Release);
                    } else {
                        self.tail.store(prev, Ordering::Release);
                    }

                    // Free the node
                    drop(Box::from_raw(current));
                } else {
                    new_len += 1;
                }

                current = next;
            }
        }

        self.len = new_len;
    }

    // Retain mut with atomic operations
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut current = self.head.load(Ordering::Acquire);
        let mut new_len = 0;

        while !current.is_null() {
            unsafe {
                let next = (*current).next.load(Ordering::Acquire);
                let should_retain = f(&mut (*current).obj);

                if !should_retain {
                    // Remove the node from the list
                    let prev = (*current).prev.load(Ordering::Acquire);
                    let next_node = next;

                    if !prev.is_null() {
                        (*prev).next.store(next_node, Ordering::Release);
                    } else {
                        self.head.store(next_node, Ordering::Release);
                    }

                    if !next_node.is_null() {
                        (*next_node).prev.store(prev, Ordering::Release);
                    } else {
                        self.tail.store(prev, Ordering::Release);
                    }

                    // Free the node
                    drop(Box::from_raw(current));
                } else {
                    new_len += 1;
                }

                current = next;
            }
        }

        self.len = new_len;
    }
}
