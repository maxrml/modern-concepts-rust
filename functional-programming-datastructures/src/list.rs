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
    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            iter: self.data.iter(),
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

impl<T> Datastructure<T> for LinkedListDS<T> where T: PartialEq + ToString + std::fmt::Display + Clone {
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
        let iter = self.data.iter().rev(); 

        for item in iter {
            acc = f(acc, item);
        }
        acc
    }
}

pub struct LinkedListIter<'a, T> {
    iter: std::collections::linked_list::Iter<'a, T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_push_front(){
        let mut list = LinkedListDS::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));

    }

    #[test]
    fn test_push_back(){
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.peek_back(), Some(&2));
    }

    #[test]
    fn test_pop_front(){
        let mut list = LinkedListDS::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back(){
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_front(3); //3 1 2 
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_peek_front(){
        let mut list: LinkedListDS<i32> = LinkedListDS::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
    }

    #[test]
    fn test_peek_back(){
        let mut list: LinkedListDS<i32> = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.peek_back(), Some(&2));
    }
   
    #[test]
    fn test_push_pop_front_back(){
        let mut list = LinkedListDS::new();
        assert!(list.is_empty());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3); //[3,2,1]
        assert_eq!(list.size(), 3);
        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_back(), Some(&1));

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(1)); //[2]
        assert_eq!(list.size(), 1);
        
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&2));
        assert_eq!(list.pop_back(), Some(2));
        assert!(list.is_empty());
    }

    #[test]
    fn test_is_empty()
    {
        let mut list: LinkedListDS<i32> = LinkedListDS::new();
        assert!(list.is_empty());

        list.push_front(1);
        assert!(!list.is_empty());

        list.pop_front();
        assert!(list.is_empty());
    }


    
    #[test]
    fn test_for_each(){

        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        let mut sum =0;
        list.for_each(|&x|{sum += x;});

        assert_eq!(sum, 1 + 2 + 3 + 4);
       
        }    

    #[test]
    fn test_size(){
        let mut list = LinkedListDS::new();
        assert_eq!(list.size(), 0);

        list.push_back(1);
        list.push_front(2);
        assert_eq!(list.size(), 2);

        list.pop_back();
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn test_to_string(){
        let mut list = LinkedListDS::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.to_string(), "[3, 2, 1]");
    }

    #[test]
    fn test_map(){

        //source_list: Quellliste mit Werten.
        let mut source_list= LinkedListDS::new();
        source_list.push_back(1);
        source_list.push_back(2);
        source_list.push_back(3);

        //target_list: Zielliste, in die die transformierten Werte eingefügt werden sollen.
        let target_list: LinkedListDS<i32> = LinkedListDS::new();
        
        //result_list: hier wird die 'map' Methode angewendet, indem jeden Wert verdoppelt.
        let result_list: LinkedListDS<i32> = source_list.map(|&x| x * 2, target_list);
   
        //expected_result: hier wird erwartet, dass die Werte in result_list verdoppelt wird.
        let mut expected_result = LinkedListDS::new();
        expected_result.push_back(2);
        expected_result.push_back(4);
        expected_result.push_back(6);

        assert_eq!(result_list.to_string(), expected_result.to_string());
   
    }

    #[test]
    fn test_lazy_map(){
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mapped: Vec<_> = list.lazy_map(|x| x*x).collect();
        assert_eq!(mapped, vec![1,4,9]);
    }

    #[test]
    fn test_filter(){
        let mut source_list = LinkedListDS::new();
        source_list.push_back(1);
        source_list.push_back(2);
        source_list.push_back(3);
        source_list.push_back(4);
        source_list.push_back(5);

        //target_list ist das Ziellist, in das die gefilterten Werte eingefügt werden sollen.
        let target_list: LinkedListDS<i32> = LinkedListDS::new();

        //result_list sammelt die gefilterten Elementen aus source_list, also die ungeraden Zahlen.
        let result_list = source_list.filter(|&x| x % 2 !=0, target_list);

        let mut expected_result = LinkedListDS::new();
        expected_result.push_back(1);
        expected_result.push_back(3);
        expected_result.push_back(5);

        assert_eq!(result_list.to_string(), expected_result.to_string())
    }
    #[test]
    fn test_lazy_filter(){
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        let filtered: Vec<&i32> = list.lazy_filter(|x| x%2 != 0).collect();
        assert_eq!(filtered, vec![&1, &3]);
    }

    #[test]
    fn test_reduce(){
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let sum = list.reduce(|acc, &x| acc + x, 0);
        assert_eq!(sum, 6);  

        //test_reduce_order
        let result = list.reduce(|acc, &x| format!("{} {}", acc, x), String::new());
        assert_eq!(result.trim(), "1 2 3");
    }

    
     #[test]
    fn test_reduce_right() {
        let mut list = LinkedListDS::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let sum = list.reduce_right(|acc, &x| acc + x, 0);
        assert_eq!(sum, 6);

        //test reduce_right_order
        let result = list.reduce_right(|acc, &x| format!("{} {}", acc, x), String::new());
        assert_eq!(result.trim(), "3 2 1");
        
    }
}


    
