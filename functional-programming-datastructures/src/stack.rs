use std::vec::Vec;
use crate::datastructures::Datastructure;

pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }
    
}

// Implementierung des Datastructure-Traits für Stack
impl<T> Datastructure<T> for Stack<T> where T: PartialEq + ToString + std::fmt::Display {
    // Gibt den Stack als String zurück
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

    // Überprüft, ob der Stack leer ist
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Gibt die Größe des Stacks zurück
    fn size(&self) -> usize {
        self.data.len()
    }
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
    fn insert(&mut self, value: T) {
        self.push(value); // `insert` nutzt einfach `push`
    }
    
}