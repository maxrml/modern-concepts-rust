mod utils;
use utils::{parallel_map_impl, parallel_reduce_impl};

pub trait Datastructure<T>: Send + Sync {
    fn is_empty(&self) -> bool;

    fn is_full(&mut self) -> bool {
        !self.is_empty()
    }

    fn size(&self) -> usize;

    fn equals(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }

    fn to_string(&self) -> String;

    fn map<U, F, D>(&self, f: F, target: D) -> D
    where
        F: FnMut(&T) -> U,
        D: Datastructure<U>;

    fn filter<F, D>(&self, f: F, target: D) -> D
    where
        F: Fn(&T) -> bool,
        D: Datastructure<T>;

    fn for_each<F>(&self, f: F)
    where
        F: FnMut(&T);

    fn reduce<U, F>(&self, f: F, initial: U) -> U
    where
         F: Fn(U, &T) -> U;

    fn reduce_right<U, F>(&self, f: F, initial: U) -> U
    where
         F: FnMut(U, &T) -> U;

    fn insert(&mut self, value: T);

    

    fn parallel_map<U, F, D>(&self, f: F, target: D) -> D
    where
        F: Fn(&T) -> U + Send + Sync + Copy +'static,
        U : Send + 'static, 
        D: Datastructure<U> + Send + 'static;
        
   
    fn parallel_reduce<U, F>(&self, f: F, initial: U) -> U
    where
        F: Fn(U, &T) -> U + Send + Sync + + Copy + 'static,
        U: Send + 'static;
    
}

    

