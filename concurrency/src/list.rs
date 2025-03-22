use rayon::prelude::*;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct List<T> {
    data : Vec<T>,
}

impl<T> List<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Send + Sync + Clone + std::fmt::Debug> List<T> {
    pub fn parallel_map<U: Send + Sync + Clone, F>(&self, f: F) -> List<U>
    where
        F: Fn(&T) -> U + Sync,
    {
        let new_data = self.data.iter().map(f).collect();
        List::new(new_data)
    }

    pub fn parallel_reduce<F>(&self, f: F) -> Option<T>
    where 
        F: Fn(T, T) -> T + Sync + Send,
        T: std::marker::Send,
    {
        if self.data.is_empty() {
            println!("Die Liste ist leer, daher ist reduce unmöglich."); 
            return None;         
        }

        println!("Daten zur Reduktion: {:?}", self.data);
        
        let output_lock = Mutex::new(());   

        Some(self.data.clone().into_par_iter().reduce(|| panic!("Leere Liste"), |a, b|{
            let _lock= output_lock.lock().unwrap();
            println!("Reduziere: {:?} + {:?} = {:?}", a, b,
            f(a.clone(), b.clone()));
            f(a, b)
        }))      
    }    
}


        