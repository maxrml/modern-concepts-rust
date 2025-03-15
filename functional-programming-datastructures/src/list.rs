use std::collections::LinkedList;
use crate::datastructures::Datastructure;
use crate::order::Order;
use crate::lazy_evaluation::Lazy;

pub struct List<T> {
    data: LinkedList<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            data: LinkedList::new(),
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.data.push_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.data.back()
    }
}

// Implementierung des Order-Traits für List
impl<T> Order<T> for List<T> {
    fn next(&mut self) -> Option<T> {
        self.pop_front()
    }
}

// Implementierung des Datastructure-Traits für List
impl<T: 'static> Datastructure<T> for List<T> 
where 
    T: PartialEq + ToString + std::fmt::Display + Clone 
{
    fn to_string(&self) -> String 
    where T: std::fmt::Display {
        let mut result = String::from("[");
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.to_string());
        }
        result.push(']');
        result
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn insert(&mut self, value: T) {
        self.push_back(value);
    }

    fn map<U: 'static, F, D>(&self, mut f: F, target: D) -> D
    where
        F: FnMut(&T) -> U,
        D: Datastructure<U>,
    {
        let mut new_target = target;
        for item in &self.data {
            new_target.insert(f(item));
        }
        new_target
    }

    fn filter<F, D>(&self, f: F, target: D) -> D
    where
        F: Fn(&T) -> bool,
        D: Datastructure<T>,
    {
        let mut new_target = target;
        for item in &self.data {
            if f(item) {
                new_target.insert(item.clone());
            }
        }
        new_target
    }

    fn for_each<F>(&self, f: F)
    where
        F: Fn(&T),
    {
        for item in &self.data {
            f(item);
        }
    }

    fn reduce<U, F>(&self, f: F, initial: U) -> U
    where
        F: Fn(U, &T) -> U,
    {
        let mut acc = initial;
        for item in &self.data {
            acc = f(acc, item);
        }
        acc
    }
    
    fn lazy_filter<F>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        let mut list_copy = List::new();
        for item in &self.data {
            list_copy.push_back(item.clone());
        }
        let boxed_order: Box<dyn Order<T>> = Box::new(list_copy);
        Lazy::new_filter(boxed_order, f)
    }
    
    fn lazy_map<F, U: 'static>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, U>
    where
        F: FnMut(&T) -> U + 'static,
    {
        let mut list_copy = List::new();
        for item in &self.data {
            list_copy.push_back(item.clone());
        }
        let boxed_order: Box<dyn Order<T>> = Box::new(list_copy);
        Lazy::new_map(boxed_order, f)
    }
}
