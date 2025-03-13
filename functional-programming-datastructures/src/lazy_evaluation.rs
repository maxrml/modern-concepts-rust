use crate::order::Order;
pub struct Lazy<S, P, T> {
    order: S,
    predicate: P,
    _marker: std::marker::PhantomData<T>,
}

impl<S, P, T> Lazy<S, P, T>
where
    S: Order<T>,
    P: Fn(&T) -> bool,
{
    pub fn new(order: S, predicate: P) -> Self {
        Lazy {
            order,
            predicate,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        while let Some(item) = self.order.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}
