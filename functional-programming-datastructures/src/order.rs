pub trait Order<T> {
    fn next(&mut self) -> Option<T>;
}
