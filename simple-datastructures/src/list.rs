use crate::stack::{Stack, Node};

#[derive(Clone)]
pub struct LinkedList<T> {
    stack: Stack<T>,    // Wir nutzen den Stack als Baustein
}

impl<T> LinkedList<T>
where T: Clone + PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            stack: Stack::new(), // Der Stack wird hier als Baustein verwendet
        }
    }

    // Gibt das Element an der angegebenen Position zurück
    pub fn get(&self, pos: i32) -> Option<&T> {
        let mut current = &self.stack.head;

        let mut index = 0;
        while let Some(node) = current {
            if index == pos {
                return Some(&node.data);
            }
            current = &node.next;
            index += 1;
        }

        None // Position existiert nicht
    }

    // Fügt mehrere Elemente am Anfang der Liste hinzu
    pub fn add(&mut self, elems: Vec<T>) {
        for elem in elems.into_iter().rev() {
            self.stack.push(elem);
        }
    }

    // Fügt ein Element an der angegebenen Position ein
    pub fn insert(&mut self, elem: T, pos: i32) {
        let mut current = &mut self.stack.head;
        let mut index = 0;

        while let Some(ref mut node) = current {
            if index == pos {
                let new_node = Box::new(Node {
                    data: elem,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                self.stack.length += 1;
                return;
            }
            current = &mut node.next;
            index += 1;
        }

        // Wenn die Position größer als die aktuelle Länge ist, fügen wir am Ende an
        self.stack.push(elem);
    }
    pub fn remove_at(&mut self, pos: i32) -> Option<T> {
        if pos == 0 {
            return self.stack.pop();
        } else {
            let mut current = &mut self.stack.head;
            let mut index = 0;
    
            // Wir durchlaufen die Liste bis zur Position
            while let Some(ref mut node) = current {
                if index == pos - 1 { // Wir müssen das Element an der Position pos entfernen
                    if let Some(mut to_remove) = node.next.take() {
                        node.next = to_remove.next.take(); // Der aktuelle Knoten wird mit dem nächsten verbunden
                        self.stack.length -= 1;
                        return Some(to_remove.data); // Das entfernte Element wird zurückgegeben
                    }
                }
                current = &mut node.next;
                index += 1;
            }
        }
        None
    }
    
    

    // Ersetzt das Element an der angegebenen Position
    pub fn replace(&mut self, elem: T, pos: i32) -> Option<T> {
        let mut current = &mut self.stack.head;
        let mut index = 0;

        while let Some(ref mut node) = current {
            if index == pos {
                let old_data = std::mem::replace(&mut node.data, elem);
                return Some(old_data);
            }
            current = &mut node.next;
            index += 1;
        }

        None // Position existiert nicht
    }

    pub fn remove(&mut self, value: T) -> Option<T> {
        let mut current = &mut self.stack.head;

        // Sonderfall: Das erste Element soll entfernt werden
        if let Some(node) = current {
            if node.data == value {
                return self.stack.pop();
            }
        }

        // Durchlaufe die Liste, um das Element zu finden
        while let Some(ref mut node) = current {
            if let Some(ref next_node) = node.next {
                if next_node.data == value {
                    if let Some(removed) = node.next.take() {
                        node.next = removed.next;
                        self.stack.length -= 1;
                        return Some(removed.data);
                    }
                }
            }
            current = &mut node.next;
        }

        None // Element wurde nicht gefunden
    }
    
    // Gibt die Größe der Liste zurück
    pub fn size(&self) -> i32 {
        self.stack.size()
    }

    // Überprüft, ob die Liste leer ist
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.stack.is_full()
    }

    // Gibt die Liste als String zurück
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        self.stack.to_string()
    }
}
