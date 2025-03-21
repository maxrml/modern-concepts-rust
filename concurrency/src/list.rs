use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct List<T> {
    data : Vec<T>,
}

impl<T> List<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Send + Sync + Clone> List<T> {
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
        T: std::marker::Send + Clone,
    {
        if self.data.is_empty() {
            panic!("Die Liste ist leer, daher ist reduce unmöglich.");
           
        }
        Some(self.data.clone().into_par_iter().reduce(|| panic!("Empty Data"), f))
    }
}
        