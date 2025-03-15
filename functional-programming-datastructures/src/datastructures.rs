use crate::order::Order;
use crate::lazy_evaluation::Lazy;

pub trait Datastructure<T: 'static> {
    fn to_string(&self) -> String where T: std::fmt::Display;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    
    fn map<U: 'static, F, D>(&self, f: F, target: D) -> D
    where
        F: FnMut(&T) -> U,
        D: Datastructure<U>;
        
    fn filter<F, D>(&self, f: F, target: D) -> D
    where
        F: Fn(&T) -> bool,
        D: Datastructure<T>;
        
    fn for_each<F>(&self, f: F)
    where
        F: Fn(&T);
        
    fn reduce<U, F>(&self, f: F, initial: U) -> U
    where
        F: Fn(U, &T) -> U;
        
    fn insert(&mut self, value: T);
    
    // Generische lazy Methoden für den Datastructure-Trait
    fn lazy_filter<F>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, T>
    where
        F: Fn(&T) -> bool + 'static;
        
    fn lazy_map<F, U: 'static>(&self, f: F) -> Lazy<Box<dyn Order<T>>, F, T, U>
    where
        F: FnMut(&T) -> U + 'static;
}