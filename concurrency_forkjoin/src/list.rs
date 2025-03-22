use rayon::join as forkjoin;

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
