use std::collections::VecDeque;
use crate::datastructures::Datastructure;

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


// Implementierung des Datastructure-Traits für Queue
impl<T> Datastructure<T> for Queue<T> where T: PartialEq + ToString + std::fmt::Display {
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
