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
    pub fn iter(&self) -> StackIter<T> {
        StackIter {
            stack: self,
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

pub struct StackIter<'a, T> {
    stack: &'a Stack<T>,
    index: usize,
}

impl<'a, T> Iterator for StackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.stack.data.len() {
            let result = &self.stack.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_lazy_map(){
        let stack = Stack{
            data: vec![1, 2, 3, 4, 5]
        };

        let mapped: Vec<_> = stack.lazy_map(|&x| x * 2).collect();
        assert_eq!(mapped, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_lazy_filter() {

        let stack = Stack {
            data: vec![1, 2, 3, 4, 5],
        };

        let filtered: Vec<_> = stack.lazy_filter(|&x| x % 2 == 0).collect();
        assert_eq!(filtered, vec![&2, &4]);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack { data: Vec::new() };

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack { data: vec![1, 2, 3] };

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.data, vec![1, 2]);

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.data, vec![1]);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.data, vec![]);

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack { data: vec![1, 2, 3] };
        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.peek(), Some(&2));
        
        stack.push(4);
        assert_eq!(stack.peek(), Some(&4));

        stack.pop();
        stack.pop();
        stack.pop();
        assert_eq!(stack.peek(), None);     
    }
 
    #[test]
    fn test_for_each() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);

        let mut sum =0;
        stack.for_each(|&x|{sum += x;});

        assert_eq!(sum, 1 + 2 + 3 + 4);
       
        }    
    

    #[test]
    fn test_reduce() {
        let stack = Stack { data: vec![1, 2, 3, 4] };
        let sum = stack.reduce(|acc, &x| acc + x, 0);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_reduce_right() {
        let stack = Stack { data: vec![1, 2, 3, 4] };
        let product = stack.reduce_right(|acc, &x| acc * x, 1);
        assert_eq!(product, 24);
    }

    #[test]
    fn test_map() {
        let stack = Stack { data: vec![1, 2, 3] };
        let mapped = stack.map(|&x| x * 2, Stack::new());
        assert_eq!(mapped.to_string(), "[2, 4, 6]");
    }

    #[test]
    fn test_filter() {
        let stack = Stack { data: vec![1, 2, 3, 4, 5] };
        let filtered = stack.filter(|&x| x % 2 == 0, Stack::new());
        assert_eq!(filtered.to_string(), "[2, 4]");
    }
    


}
