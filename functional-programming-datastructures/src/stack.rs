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

impl<T> Datastructure<T> for Stack<T> where T: PartialEq + ToString + std::fmt::Display + Clone {

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
        self.push(value); 
    }

    fn map<U, F, D>(&self, mut f: F, target: D) -> D
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

    fn reduce_right<U, F>(&self, mut f: F, initial: U) -> U
    where
        F: FnMut(U, &T) -> U,
    {
        let mut acc = initial;
        let len = self.data.len();

        for i in (0..len).rev() { 
            acc = f(acc, &self.data[i]);
        }
        acc 
    }
    
}