use crate::datastructure::Datastructure;

#[derive(Debug)]
pub struct Node<T> {
    pub content: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    #[allow(dead_code)] // avoid warning
    pub fn new(content: T) -> Self {
        Node {
            content,
            next: None,
        }
    }
}
#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

// Implementierung Fkt ohne Trait
impl<T: std::fmt::Display + PartialEq + Copy> LinkedList<T> { // Display ist für eine angenehm formatierte Ausgabe
    /// Erzeugt eine neue leere Liste (Konstruktor)
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Ersetzt den Inhalt des Elements am angegebenen Index
    pub fn replace(&mut self, index: usize, element: T) {
        let mut current = match self.head.as_mut() {
            Some(node) => node,
            None => return,
        };
        for _ in 0..index { // von 0 bis Index
            if let Some(next) = current.next.as_mut() {
                current = next;
            } else {
                current.content = element;
                return;
            }
        }
        current.content = element;
    }

    /// Fügt ein neues Element an der angegebenen Stelle ein
    pub fn insert(&mut self, index: usize, element: T) {
        // Wenn index 0 Aufruf der add_first Funktion
        if index == 0 {
            self.add_first(element);
            return;
        }
        let mut current = match self.head.as_mut() {
            Some(node) => node,
            None => return,
        };
        for _ in 0..(index - 1) {
            if let Some(next) = current.next.as_mut() {
                current = next;
            } else {
                return;
            }
        }
        let new_node = Box::new(Node {
            content: element,
            next: current.next.take(),
        });
        current.next = Some(new_node);
    }

    /// Fügt ein neues Element am Anfang der Liste hinzu
    pub fn add_first(&mut self, element: T) {
        let new_node = Box::new(Node {
            content: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// Hängt ein neues Element am Ende der Liste an
    pub fn add(&mut self, element: T) {
        let new_node = Box::new(Node {
            content: element,
            next: None,
        });
        match self.head.as_mut() {
            None => self.head = Some(new_node),
            Some(mut node) => {
                while let Some(ref mut next) = node.next {
                    node = next;
                }
                node.next = Some(new_node);
            }
        }
    }

    /// Entfernt das erste Element der Liste und gibt dessen Inhalt zurück
    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.content
        })
    }

    /// Entfernt das Element an der angegebenen Stelle
    pub fn remove_at(&mut self, index: usize) {
        if index == 0 {
            self.head = self.head.take().and_then(|node| node.next);
            return;
        }
        let mut current = match self.head.as_mut() {
            Some(node) => node,
            None => return,
        };
        for _ in 0..(index - 1) {
            if let Some(next) = current.next.as_mut() {
                current = next;
            } else {
                return;
            }
        }
        if let Some(next_node) = current.next.take() {
            current.next = next_node.next;
        }
    }

    /// Entfernt das erste Element, dessen Inhalt dem gesuchten Wert entspricht und gibt dessen Inhalt zurück
    pub fn remove(&mut self, element: &T) -> Option<T> {
        if let Some(ref head) = self.head {
            if head.content == *element {
                return self.remove_first();
            }
        } else {
            return None;
        }
        let mut current = self.head.as_mut().unwrap();
        while let Some(ref mut next_node) = current.next {
            if next_node.content == *element {
                let removed = current.next.take();
                if let Some(mut removed_node) = removed {
                    current.next = removed_node.next.take();
                    return Some(removed_node.content);
                }
            } else {
                if current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                } else {
                    break;
                }
            }
        }
        println!("Could not find node with content '{}'", element);
        None
    }

    /// Überprüft, ob die Liste ein Element mit dem gesuchten Inhalt enthält
    pub fn get(&self, element: &T) -> bool {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if node.content == *element {
                return true;
            }
            current = node.next.as_ref();
        }
        false
    }

    /// Gibt den Inhalt des Elements am angegebenen Index zurück
    pub fn content(&self, index: usize) -> Option<T> {
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current?.next.as_ref();
        }
        current.map(|node| node.content)
    }
}




impl<T> Datastructure<T> for LinkedList<T> 
where 
T: std::fmt::Display + PartialEq + Copy,
{
    
    /// Gibt eine String-Repräsentation der Liste zurück
    fn to_string(&self) -> String
    where
        T: ToString,
    {
        let mut s = String::new();
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            s.push_str(&node.content.to_string());
            s.push_str(" ");
            current = node.next.as_ref();
        }
        s.trim_end().to_string()
    }

    /// Prüft, ob die Liste leer ist
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    

    /// Gibt die Anzahl der Elemente in der Liste zurück
    fn size(&self) -> i32 {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }

 


}

// ------------------------------Testing--------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_linked_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }


    #[test]
    fn test_add_first() {
        let mut list = LinkedList::new();
        list.add_first(10);
        assert_eq!(list.size(), 1);
        assert_eq!(list.content(0), Some(10));
    }

    #[test]
    fn test_add() {
        let mut list = LinkedList::new();
        list.add(20);
        list.add(30);
        assert_eq!(list.size(), 2);
        assert_eq!(list.content(0), Some(20));
        assert_eq!(list.content(1), Some(30));
    }

    #[test]
    fn test_remove_first() {
        let mut list = LinkedList::new();
        list.add(40);
        list.add(50);
        let removed = list.remove_first();
        assert_eq!(removed, Some(40));
        assert_eq!(list.size(), 1);
        assert_eq!(list.content(0), Some(50));
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.add(60);
        list.add(70);
        list.insert(1, 65);
        assert_eq!(list.size(), 3);
        assert_eq!(list.content(0), Some(60));
        assert_eq!(list.content(1), Some(65));
        assert_eq!(list.content(2), Some(70));
    }

    #[test]
    fn test_replace() {
        let mut list = LinkedList::new();
        list.add(80);
        list.add(90);
        list.replace(1, 85);
        assert_eq!(list.size(), 2);
        assert_eq!(list.content(0), Some(80));
        assert_eq!(list.content(1), Some(85));
    }


    #[test]
    fn test_remove_at() {
        let mut list = LinkedList::new();
        list.add(100);
        list.add(110);
        list.add(120);
        list.remove_at(1);
        assert_eq!(list.size(), 2);
        assert_eq!(list.content(0), Some(100));
        assert_eq!(list.content(1), Some(120));
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        list.add(130);
        list.add(140);
        list.add(150);
        let removed = list.remove(&140);
        assert_eq!(removed, Some(140));
        assert_eq!(list.size(), 2);
        assert_eq!(list.content(0), Some(130));
        assert_eq!(list.content(1), Some(150));
    }

    #[test]
    fn test_remove_none() {
        let mut list = LinkedList::new();
        list.add(130);
        list.add(140);
        list.add(150);
        assert_eq!(list.remove(&120), None);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.add(160);
        list.add(170);
        assert!(list.get(&160));
        assert!(!list.get(&180));
    }

    #[test]
    fn test_to_string() {
        let mut list = LinkedList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        assert_eq!(list.to_string(), "1 2 3");
    }
}