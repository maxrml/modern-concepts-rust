pub trait Datastructure<T> {
    fn is_empty(&self) -> bool;

    fn is_full(&mut self) -> bool {
        !self.is_empty()
    }

    fn size(&self) -> usize;

    fn equals(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }

    fn to_string(&self) -> String;
}
