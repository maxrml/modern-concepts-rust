
use datastructure::Datastructure;


#[derive(Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Stack<T> {
    pub head: Option<Box<Node<T>>>,
    pub length: i32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
            length: 0,
        }
    }


    // Gibt die Größe der Liste zurück
    pub fn size(&self) -> i32 {
        self.length
    }

    
    }
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    // Pop-Funktion: Entfernt das oberste Element vom Stack und gibt es zurück
    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.length -= 1;
            Some(node.data)
        } else {
            None // Wenn der Stack leer ist, geben wir None zurück
        }
    }




impl datastructure::Datastructure for <T> Stack<T> {

    pub fn equals(&self, other: &Stack<T>) -> bool
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

    // Überprüft, ob die Liste leer ist
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Überprüft, ob die Liste voll ist
    pub fn is_full(&self) -> bool {
        self.length != 0


}
}


