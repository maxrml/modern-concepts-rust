use std::sync::{Arc, Mutex};
use std::thread;
use num_cpus;
use crate::datastructures::Datastructure;

pub fn parallel_map_impl<T, U, F, D>(data: &Vec<T>, f: F, mut target: D) -> D
where
        F: Fn(&T) -> U + Send + Sync + Copy + 'static, 
        U: Send + 'static,
        D: Datastructure<U> + Send + 'static + std::fmt::Debug,
        T: Send + Sync + 'static,
{
        let num_threads = num_cpus::get();
        let chunk_size = (data.len() + num_threads - 1) / num_threads;
        let target = Arc::new(Mutex::new(target));

        thread::scope(|s| {
            for chunk in data.chunks(chunk_size){
                let target_clone = Arc::clone(&target);
                s.spawn(move || {
                    let transformed_chunk: Vec<U> = chunk.iter().map(f).collect();
                    let mut locked_target = target_clone.lock().unwrap();
                    for item in transformed_chunk {
                        locked_target.insert(item);
                    }
                });
            }
        }.unwrap());

        Arc::try_unwrap(target).unwrap().into_inner().unwrap()
}

pub fn parallel_reduce_impl<T, U, F>(data: &Vec<T>, f: F, initial: U) -> U
where 
    F: Fn(U, &T) -> U + Send + Sync + 'static,
    U: Send + 'static + Clone,
{
    if data.is_empty(){
        panic!("Cannot reduce an empty data structure");
    }
        
    let num_threads = num_cpus::get();
    let chunk_size = (data.len() + num_threads - 1) / num_threads;

    let mut results = Vec::new();
    thread::scope(|s| {
        for chunk in data.chunks(chunk_size) {
            results.push(s.spawn(move || {
            chunk.iter().fold(initial.clone(), |acc, &T| f(acc, item))
            }));    
        }
    }).unwrap();

    results.into_iter().map(|r| r.join().unwrap()).fold(initial, f)
}


