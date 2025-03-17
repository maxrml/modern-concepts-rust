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

    fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
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

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_enqueue(){
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.size(), 2);
        assert_eq!(queue.get(0), Some(&2));
        assert_eq!(queue.get(1), Some(&1));
    }

    #[test]
    fn test_dequeue(){
        let mut queue= Queue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.size(), 2);
        queue.dequeue();
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.dequeue(), Some(2));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_map(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let target_queue: Queue<i32> = Queue::new();
        let result_queue = queue.map(|x| x * x, target_queue);
        
        assert_eq!(result_queue.get(0), Some(&1));
        assert_eq!(result_queue.get(1), Some(&4));
        assert_eq!(result_queue.get(2), Some(&9));

    }
    #[test]
    fn test_lazy_map(){

        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let mapped_results : Vec<_> = queue.lazy_map(|&x| x * 2).collect();
        assert_eq!(mapped_results, vec![6, 4, 2]);
    }

    #[test]
    fn test_lazy_filter(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);

        let filtered_results: Vec<_> = queue.lazy_filter(|&x| x % 2 != 0).collect();
        assert_eq!(filtered_results, vec![&5, &3, &1]);
    }

    #[test]
    fn test_get(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        //Test get mit den gültigen Indexabfragen
        assert_eq!(queue.get(0), Some(&3));
        assert_eq!(queue.get(1), Some(&2));
        assert_eq!(queue.get(2), Some(&1));

        //Test get mit den ungültigen Indexabfragen
        assert_eq!(queue.get(4), None);

        //Test get mit empty queue
        queue.dequeue();
        queue.dequeue();
        queue.dequeue();
        assert_eq!(queue.get(0), None);

    }

    #[test]
    fn test_filter(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        let target_queue: Queue<i32> = Queue::new();
        let result_queue = queue.filter(|x| x % 2 == 0, target_queue);
        assert_eq!(result_queue.get(0), Some(&2));
        assert_eq!(result_queue.get(1), Some(&4));

    }
    

    #[test]
    fn test_to_string() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        let result_string = queue.to_string();
        assert_eq!(result_string, "[4, 3, 2, 1]");
    }

    
    #[test]
    fn test_for_each_to_string() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);

    let mut sum =0;
        queue.for_each(|&x|{sum += x;});

        assert_eq!(sum, 1 + 2 + 3 + 4);
       
    }
    


    #[test]
    fn test_reduce(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let sum = queue.reduce(|acc, &x| acc + x, 0);
        assert_eq!(sum, 6);  

        let result = queue.reduce(|acc, &x| format!("{} {}", acc, x), String::new());
        assert_eq!(result.trim(), "3 2 1");
    }

    #[test]
    fn text_reduce_right(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let sum = queue.reduce_right(|acc, &x| acc + x, 0);
        assert_eq!(sum, 6);

        let result = queue.reduce_right(|acc, &x| format!("{} {}", acc, x), String::new());
        assert_eq!(result.trim(), "1 2 3");
    }

}




