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
    pub fn iter(&self) -> QueueIter<T> {
        QueueIter {
            queue: self,
            index: 0,
        }
    }

    pub fn lazy_map<'a, U, F>(&'a self, f: F) -> impl Iterator<Item = U> + 'a
    where
        F: Fn(&T) -> U + 'a,
    {
        self.iter().map(f)
    }

    pub fn lazy_filter<'a, F>(&'a self, mut f: F) -> impl Iterator<Item = &T> + 'a
    where
        F: FnMut(&T) -> bool + 'a, 
    {
        self.iter().filter(move |x| f(x))
    }

    pub fn enqueue(&mut self, value: T) {
        self.data.push_front(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }  
}


impl<T> Datastructure<T> for Queue<T> where T: PartialEq + ToString + std::fmt::Display + Clone {

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
        self.enqueue(value); 
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

pub struct QueueIter<'a, T> {
    queue: &'a Queue<T>,
    index: usize,
}

impl<'a, T> Iterator for QueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.queue.data.len() {
            let result = self.queue.data.get(self.index);
            self.index += 1;
            result
        } else {
            None
        }
    }
}