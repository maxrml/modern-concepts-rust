pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}


pub struct List<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    length: i32,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn equals(&self, other: &List<T>) -> bool
    where
        T: PartialEq,
    {
        if self.length != other.length {
            return false;
        }

        let mut current_self = &self.head;
        let mut current_other = &other.head;

        while let (Some(node_self), Some(node_other)) = (current_self, current_other) {
            if node_self.data != node_other.data {
                return false;
            }
            current_self = &node_self.next;
            current_other = &node_other.next;
        }

        true
    }

    // Gibt die Liste als String zurück
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        let mut result = String::new();
        let mut current = &self.head;

        while let Some(node) = current {
            result.push_str(&node.data.to_string());
            if node.next.is_some() {
                result.push_str(" -> ");
            }
            current = &node.next;
        }

        result
    }

    // Gibt die Größe der Liste zurück
    pub fn size(&self) -> i32 {
        self.length
    }

    // Überprüft, ob die Liste leer ist
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Überprüft, ob die Liste voll ist
    pub fn is_full(&self) -> bool {
        self.length != 0
    }
}
