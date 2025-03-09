use std::fmt::Display;

// Definiere die Node-Struktur
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Definiere die LinkedList-Struktur
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Display,
{
    // Erstellt eine neue leere LinkedList
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    // Gibt die Anzahl der Elemente in der Liste zurück
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(_) = current {
            count += 1;
            current = &current.as_ref().unwrap().next;
        }
        count
    }

    
   
    // Füge ein Element am Ende der Liste hinzu
    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        
        
    }


    // Gibt eine String-Repräsentation der Liste zurück
    pub fn to_string(&self) -> String {
        let mut elements = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            elements.push(format!("{}", node.value));
            current = &node.next;
        }
        format!("LinkedList [{}]", elements.join(", "))
    }

}

