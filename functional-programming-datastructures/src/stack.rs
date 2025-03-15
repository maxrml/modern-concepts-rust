use std::vec::Vec;
use crate::order::Order;
use crate::datastructures::Datastructure;
use crate::lazy_evaluation::Lazy;

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

// Implementierung des Order-Traits für Stack
impl<T> Order<T> for Stack<T> {
    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

// Implementierung des Datastructure-Traits für Stack
// Hier fügen wir 'static zu T hinzu, um das Lifetime-Problem zu lösen
impl<T: 'static> Datastructure<T> for Stack<T> 
where 
    T: PartialEq + ToString + std::fmt::Display + Clone
{
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
    
    fn map<U: 'static, F, D>(&self, mut f: F, target: D) -> D
    where
        F: FnMut(&T) -> U,
        D: Datastructure<U>,
    {
        let mut new_target = target;
        for item in &self.data {
            let transformed = f(item);
            new_target.insert(transformed);
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

    fn insert(&mut self, value: T) {
        self.push(value);
    }

    fn lazy_filter<F>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        // Erstellen einer Kopie des Stacks, um die Originaldaten nicht zu verändern
        let mut stack_copy = Stack::new();
        for item in &self.data {
            stack_copy.push(item.clone());
        }
        
        // Stack in eine Box verpacken, um ihn als Trait-Objekt zu nutzen
        let boxed_order: Box<dyn Order<T>> = Box::new(stack_copy);
        Lazy::new_filter(boxed_order, f)
    }
    
    fn lazy_map<F, U: 'static>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, U>
    where
        F: FnMut(&T) -> U + 'static,
    {
        // Erstellen einer Kopie des Stacks, um die Originaldaten nicht zu verändern
        let mut stack_copy = Stack::new();
        for item in &self.data {
            stack_copy.push(item.clone());
        }
        
        // Stack in eine Box verpacken, um ihn als Trait-Objekt zu nutzen
        let boxed_order: Box<dyn Order<T>> = Box::new(stack_copy);
        Lazy::new_map(boxed_order, f)
    }
}