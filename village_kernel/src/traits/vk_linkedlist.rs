//###########################################################################
// vk_list.rs
// Declarations of the list
//
// $Copyright: Copyright (C) village
//###########################################################################
extern crate alloc;
use alloc::boxed::Box;
use core::ptr;
use core::iter::Iterator;
use core::marker::PhantomData;

// List node struct
struct ListNode<T> {
    obj: T,
    prev: *mut ListNode<T>,
    next: *mut ListNode<T>,
}

// List template 
pub struct LinkedList<T> {
    head: *mut ListNode<T>,
    tail: *mut ListNode<T>,
    iter: *mut ListNode<T>,
    len: i32,
}

// Impl list
impl<T> LinkedList<T> {
    // New
    pub const fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            iter: ptr::null_mut(),
            len: 0,
        }
    }

    // List get len
    pub fn len(&self) -> i32 {
        self.len
    }
}

impl<T> LinkedList<T> {
    // Push node to list
    pub fn add(&mut self, obj: T){
        let node = ListNode {
            obj,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        };
        
        let node_ptr = Box::into_raw(Box::new(node));
        
        if self.head.is_null() {
            self.head = node_ptr;
            self.tail = node_ptr;
        } else {
            unsafe {
                (*self.tail).next = node_ptr;
                (*node_ptr).prev = self.tail;
                self.tail = node_ptr;
            }
        }
        
        self.len += 1;
    }

    // Delete node from list
    pub fn del(&mut self, obj: &T)
    where
        T: PartialEq,
    {
        let mut current = self.head;
        
        while !current.is_null() {
            unsafe {
                let node = &*current;
                let obj_eq = &node.obj == obj;

                if obj_eq {
                    let prev = node.prev;
                    let next = node.next;
                    
                    if !prev.is_null() {
                        (*prev).next = next;
                    } else {
                        self.head = next;
                    }
                    
                    if !next.is_null() {
                        (*next).prev = prev;
                    } else {
                        self.tail = prev;
                    }
                    
                    // Free the node
                    drop(Box::from_raw(current));
                    self.len -= 1;
                }
                
                current = node.next;
            }
        }
    }

    // Clear List
    pub fn clear(&mut self) {
        let mut current = self.head;
        
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                drop(Box::from_raw(current));
                current = next;
            }
        }
        
        self.head = ptr::null_mut();
        self.tail = ptr::null_mut();
        self.len = 0;
    }
}

// LinkedList self iterator
impl<T> LinkedList<T> {
    // List begin node
    pub fn begin(&mut self) {
        self.iter = self.head;
    }

    // List next node
    pub fn next(&mut self) {
        if !self.iter.is_null() {
            unsafe {
                self.iter = (*self.iter).next;
            }
        }
    }

    // List prev node
    pub fn prev(&mut self) {
        if !self.iter.is_null() {
            unsafe {
                self.iter = (*self.iter).prev;
            }
        }
    }

    // List end node
    pub fn end(&mut self) {
        self.iter = self.tail;
    }

    // List is begin
    pub fn is_begin(&self) -> bool {
        self.iter.is_null()
    }

    // List is end
    pub fn is_end(&self) -> bool {
        self.iter.is_null()
    }

    // List is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // List item
    pub fn item(&self) -> Option<&mut T> {
        if self.iter.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.iter).obj) }
        }
    }
}

// Impl drop for linked list
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

// List iterator
pub struct ListIterator<'a, T> {
    _list: &'a LinkedList<T>,
    current: *mut ListNode<T>,
    marker: PhantomData<&'a mut T>,
}

// List reverse iterator
pub struct ListReverseIterator<'a, T> {
    _list: &'a LinkedList<T>,
    current: *mut ListNode<T>,
    marker: PhantomData<&'a mut T>,
}

// Impl iterator for list iterator
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let obj = &mut (*self.current).obj;
                self.current = (*self.current).next;
                Some(obj)
            }
        }
    }
}

// Impl iterator for list reverse iterator
impl<'a, T> Iterator for ListReverseIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let obj = &mut (*self.current).obj;
                self.current = (*self.current).prev;
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
            current: self.head,
            marker: PhantomData,
        }
    }

    // Create reverse iterator
    pub fn rev_iter_mut(&mut self) -> ListReverseIterator<'_, T> {
        ListReverseIterator {
            _list: self,
            current: self.tail,
            marker: PhantomData,
        }
    }
}

// Impl linked list
impl<T> LinkedList<T> {
    /// Retains only the elements specified by the predicate.
    /// In other words, remove all elements `e` for which `f(&e)` returns false.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut current = self.head;
        let mut new_len = 0;

        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                let should_retain = f(&(*current).obj);

                if !should_retain {
                    // Remove the node from the list
                    let prev = (*current).prev;
                    let next = (*current).next;

                    if !prev.is_null() {
                        (*prev).next = next;
                    } else {
                        self.head = next;
                    }

                    if !next.is_null() {
                        (*next).prev = prev;
                    } else {
                        self.tail = prev;
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

    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut current = self.head;
        let mut prev: *mut ListNode<T> = ptr::null_mut();
        let mut new_len = 0;

        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                let should_retain = f(&mut (*current).obj);

                if should_retain {
                    (*current).prev = prev;
                    if prev.is_null() {
                        self.head = current;
                    } else {
                        (*prev).next = current;
                    }
                    prev = current;
                    new_len += 1;
                } else {
                    drop(Box::from_raw(current));
                }

                current = next;
            }
        }

        self.tail = prev;
        if !prev.is_null() {
            unsafe { (*prev).next = ptr::null_mut(); }
        }
        self.len = new_len;
    }
}
