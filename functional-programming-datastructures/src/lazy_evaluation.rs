use std::marker::PhantomData;
use crate::order::Order;

pub struct Lazy<S, F, T, U> {
    order: S,
    function: F,
    _marker_t: PhantomData<T>,
    _marker_u: PhantomData<U>,
}

// Implementierung für Filter-Operationen
impl<S, P, T> Lazy<S, P, T, T>
where
    S: Order<T>,
    P: Fn(&T) -> bool,
{
    pub fn new_filter(order: S, predicate: P) -> Self {
        Lazy {
            order,
            function: predicate,
            _marker_t: PhantomData,
            _marker_u: PhantomData,
        }
    }

    pub fn next_filter(&mut self) -> Option<T> {
        while let Some(item) = self.order.next() {
            if (self.function)(&item) {
                return Some(item);
            }
        }
        None
    }
}

// Implementierung für Map-Operationen
impl<S, F, T, U> Lazy<S, F, T, U>
where
    S: Order<T>,
    F: FnMut(&T) -> U,
{
    pub fn new_map(order: S, transform: F) -> Self {
        Lazy {
            order,
            function: transform,
            _marker_t: PhantomData,
            _marker_u: PhantomData,
        }
    }

    pub fn next_map(&mut self) -> Option<U> {
        if let Some(item) = self.order.next() {
            return Some((self.function)(&item));
        }
        None
    }
}