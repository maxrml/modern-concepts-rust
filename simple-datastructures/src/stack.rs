use crate::datastructure::Datastructure;

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

    
    // Fügt ein Element oben auf den Stack
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
    
    // Gibt das oberste Element zurück, ohne es zu entfernen
    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.data),
            None => None,// Wenn der Stack leer ist, geben wir None zurück
        }
    }
}





// Implementierung des Datastructure-Traits
impl<T> Datastructure for Stack<T>
where
    T: PartialEq + ToString,
{
    // Vergleicht zwei Datenstrukturen auf Gleichheit
    fn equals(&self, other: &Self) -> bool {
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

    // Gibt den Stack als String zurück
    fn to_string(&self) -> String {
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

    // Überprüft, ob der Stack leer ist
    fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Gibt die Größe des Stacks zurück
    fn size(&self) -> i32 {
        self.length
    } 
}

