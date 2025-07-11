//###########################################################################
// vk_list.rs
// Declarations of the list with atomic pointers
//
// $Copyright: Copyright (C) village
//###########################################################################
use alloc::boxed::Box;
use core::iter::{Iterator, DoubleEndedIterator};
use core::marker::PhantomData;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Struct ListNode
struct ListNode<T> {
    obj: T,
    prev: AtomicPtr<ListNode<T>>,
    next: AtomicPtr<ListNode<T>>,
}

// Struct LinkedList
pub struct LinkedList<T> {
    head: AtomicPtr<ListNode<T>>,
    tail: AtomicPtr<ListNode<T>>,
    iter: AtomicPtr<ListNode<T>>,
    len: usize,
}

// Impl LinkedList
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

    // Push node to list tail
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

    // Clear List
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

    // List get len
    pub fn len(&self) -> usize {
        self.len
    }
}

// LinkedList self iterator
impl<T> LinkedList<T> {
    // List next node
    pub fn cycle(&mut self) -> Option<&mut T> {
        unsafe {
            let iter_ptr = self.iter.load(Ordering::Acquire);
            let mut next_ptr: *mut ListNode<T> = ptr::null_mut();
            if !iter_ptr.is_null() {
                next_ptr = (*iter_ptr).next.load(Ordering::Acquire);
            }
            if next_ptr.is_null() {
                next_ptr = self.head.load(Ordering::Acquire);
            }
            self.iter.store(next_ptr, Ordering::Release);
            Some(&mut (*next_ptr).obj)
        }
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

// Linkedlist mutable iterator
pub struct ListIterMut<'a, T> {
    current: AtomicPtr<ListNode<T>>,
    current_back: AtomicPtr<ListNode<T>>,
    marker: PhantomData<&'a mut T>,
}

// Impl iterator for list mutable iterator
impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.load(Ordering::Acquire);
        if current.is_null() {
            None
        } else {
            unsafe {
                let next = (*current).next.load(Ordering::Acquire);
                self.current.store(next, Ordering::Release);
                Some(&mut (*current).obj)
            }
        }
    }
}

// Impl double ended iterator for list mutable iterator
impl<'a, T> DoubleEndedIterator for ListIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_back = self.current_back.load(Ordering::Acquire);
        if current_back.is_null() {
            None
        } else {
            unsafe {
                let prev = (*current_back).prev.load(Ordering::Acquire);
                self.current_back.store(prev, Ordering::Release);
                Some(&mut (*current_back).obj)
            }
        }
    }
}

// Impl LinkedList
impl<T> LinkedList<T> {
    // Create mutable iterator
    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        ListIterMut {
            current: AtomicPtr::new(head),
            current_back: AtomicPtr::new(tail),
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
