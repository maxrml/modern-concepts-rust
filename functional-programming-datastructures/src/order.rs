// Definition des Order-Traits
pub trait Order<T> {
    fn next(&mut self) -> Option<T>;
}

// Implementierung von Order für Box<dyn Order<T>>
impl<T> Order<T> for Box<dyn Order<T>> {
    fn next(&mut self) -> Option<T> {
        (**self).next()
    }
}