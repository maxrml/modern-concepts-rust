use std::collections::LinkedList;
use crate::datastructures::Datastructure;

pub struct LinkedListDS<T> {
    data: LinkedList<T>,
}

impl<T> LinkedListDS<T> {
    pub fn new() -> Self {
        LinkedListDS {
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

// Implementierung des Datastructure-Traits für LinkedListDS
impl<T> Datastructure<T> for LinkedListDS<T> where T: PartialEq + ToString + std::fmt::Display {
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

    // Wendet eine Funktion auf alle Elemente an und speichert sie in der Zieldatenstruktur
    fn map<U, F, D>(&self, mut f: F, target: D) -> D
    where
        F: FnMut(&T) -> U,
        D: Datastructure<U>,
    {
        let mut new_target = target;
        for item in &self.data {
            let transformed = f(item); // Hier wird `f` als mutabel genutzt
            new_target.insert(transformed);
        }
        new_target
    }
    
}
