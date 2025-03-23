use rayon::join as forkjoin;
use std::time::Instant;

// Parallel Map using ForkJoin
pub fn parallel_map<T, U, F>(data: &[T], f: &F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync, U: Clone
{
    if data.is_empty() {
        return Vec::new();
    }

    if data.len() == 1 {
        return vec![f(&data[0])];
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);

    let (left_result, right_result) = forkjoin(
        || parallel_map(left, f),
        || parallel_map(right, f),
    );

    [left_result, right_result].concat()
}

// Parallel Reduce using ForkJoin
pub fn parallel_reduce<T, F>(data: &[T], f: &F) -> T
where
    T: Sync + Send + Clone,
    F: Fn(&T, &T) -> T + Sync,
{
    if data.is_empty() {
        panic!("Cannot reduce an empty data structure");
    }

    if data.len() == 1 {
        return data[0].clone();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at(mid);


    let (left_result, right_result) = forkjoin(
        || parallel_reduce(left, f),
        || parallel_reduce(right, f),
    );
   
    f(&left_result, &right_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_map() {
        let mut list = vec![1, 2, 3, 4, 5];

        let mapped_data = parallel_map(&list, &|x| x * 2);
        assert_eq!(mapped_data, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_reduce() {
        let mut list = vec![1, 2, 3, 4, 5];

        let data = parallel_reduce(&list, &|a, b| a + b);
        assert_eq!(data, 15);
    }

    #[test]
    fn test_parallel_map_empty() {
        let data: Vec<u32> = vec![];
        let result = parallel_map(&data, &|x| x * 2);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parallel_map_single_element() {
        let data = vec![42];
        let result = parallel_map(&data, &|x| x * 2);
        assert_eq!(result, vec![84]);
    }

    #[test]
    fn test_parallel_reduce_empty() {
        let data: Vec<u32> = vec![];
        let result = std::panic::catch_unwind(|| parallel_reduce(&data, &|x, y| x + y));
        assert!(result.is_err());
    }

    #[test]
    fn test_parallel_reduce_single_element() {
        let data = vec![42];
        let result = parallel_reduce(&data, &|x, y| x + y);
        assert_eq!(result, 42); 
    }

    //Ausgabe anzeigen: cargo test -- --nocapture
    #[test]
    fn test_parallel_map_with_timing() {
        let list: Vec<u32> = (1..=1_000_000).collect(); // Deutlich größerer Vektor
        
        let start = Instant::now();
        let mapped_data = parallel_map(&list, &|x| x * 2);
        let duration = start.elapsed();
        
        assert_eq!(mapped_data.len(), list.len());
        assert_eq!(mapped_data[0], 2);
        assert_eq!(mapped_data[list.len() - 1], list[list.len() - 1] * 2);

        println!("Parallel map execution time: {:?}", duration);
    }


} 

