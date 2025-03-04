use crate::datastructure::List;
pub struct Stack<T> {
    list: List<T>,
}

impl<T> Stack<T> {
    // Konstruktor für eine leere LinkedList
    pub fn new() -> Self {
        Stack {
            list: List::new(),
        }
    }

    // `equals` ruft die `equals`-Methode der inneren `List` auf
    pub fn equals(&self, other: &Stack<T>) -> bool
    where
        T: PartialEq,
    {
        self.list.equals(&other.list)
    }

    // `to_string` ruft die `to_string`-Methode der inneren `List` auf
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        self.list.to_string()
    }

    // `size` ruft die `size`-Methode der inneren `List` auf
    pub fn size(&self) -> i32 {
        self.list.size()
    }

    // `is_empty` ruft die `is_empty`-Methode der inneren `List` auf
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    // `is_full` ruft die `is_full`-Methode der inneren `List` auf
    pub fn is_full(&self) -> bool {
        self.list.is_full()
    }

    // Hier könnte eine Methode hinzugefügt werden, um Elemente hinzuzufügen oder zu entfernen
}