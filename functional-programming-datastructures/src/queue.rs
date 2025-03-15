use std::collections::VecDeque;
use crate::datastructures::Datastructure;
use crate::order::Order;
use crate::lazy_evaluation::Lazy;

pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }  
}

// Implementieren des Order-Traits für Queue
impl<T> Order<T> for Queue<T> {
    fn next(&mut self) -> Option<T> {
        self.dequeue()
    }
}

// Implementierung des Datastructure-Traits für Queue
impl<T: 'static> Datastructure<T> for Queue<T> 
where 
    T: PartialEq + ToString + std::fmt::Display + Clone 
{
    // Gibt die Queue als String zurück
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

    // Überprüft, ob die Queue leer ist
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Gibt die Größe der Queue zurück
    fn size(&self) -> usize {
        self.data.len()
    }
    
    // Fügt ein Element in die Queue ein (am Ende)
    fn insert(&mut self, value: T) {
        self.enqueue(value); // Nutzt die bestehende `enqueue`-Methode
    }

    // Wendet eine Funktion auf alle Elemente an und speichert sie in der Zieldatenstruktur
    fn map<U: 'static, F, D>(&self, mut f: F, target: D) -> D
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
    
    // Implementierung der lazy_filter Methode
    fn lazy_filter<F>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, T>
    where
        F: Fn(&T) -> bool + 'static,
    {
        // Erstellen einer Kopie der Queue, um die Originaldaten nicht zu verändern
        let mut queue_copy = Queue::new();
        for item in &self.data {
            queue_copy.enqueue(item.clone());
        }
        
        // Queue in eine Box verpacken, um sie als Trait-Objekt zu nutzen
        let boxed_order: Box<dyn Order<T>> = Box::new(queue_copy);
        Lazy::new_filter(boxed_order, f)
    }
    
    // Implementierung der lazy_map Methode
    fn lazy_map<F, U: 'static>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, U>
    where
        F: FnMut(&T) -> U + 'static,
    {
        // Erstellen einer Kopie der Queue, um die Originaldaten nicht zu verändern
        let mut queue_copy = Queue::new();
        for item in &self.data {
            queue_copy.enqueue(item.clone());
        }
        
        // Queue in eine Box verpacken, um sie als Trait-Objekt zu nutzen
        let boxed_order: Box<dyn Order<T>> = Box::new(queue_copy);
        Lazy::new_map(boxed_order, f)
    }
}