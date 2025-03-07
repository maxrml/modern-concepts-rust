// Definition eines Knotens
#[derive(Debug)]
pub struct Node<T> {
    pub content: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(content: T) -> Self {
        Node {
            content,
            next: None,
        }
    }
}

// Definition der verketteten Liste (LinkedList)
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug + PartialEq + Copy> LinkedList<T> {
    /// Konstruktor für eine leere Liste
    pub fn new() -> Self {
        LinkedList { head: None }
    }


    /// Gibt die Liste als String wieder
    pub fn to_string(&self) -> String
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

    /// Prüft, ob die Liste leer ist.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Gibt die Anzahl der Elemente in der Liste zurück
    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }

    /// Gibt den Inhalt des Elements am angegebenen Index zurück
    pub fn content(&self, index: usize) -> Option<&T> {
        let mut current = self.head.as_ref();
        for _ in 0..index {
            current = current?.next.as_ref();
        }
        current.map(|node| &node.content)
    }

    /// Ersetzt den Inhalt des Elements am angegebenen Index durch den neuen Wert
    pub fn replace(&mut self, index: usize, element: T) {
        let mut current = match self.head.as_mut() {
            Some(node) => node,
            None => return,
        };
        for _ in 0..index {
            if let Some(next) = current.next.as_mut() {
                current = next;
            } else {
                // Wenn der Index außerhalb liegt, wird der Inhalt des letzten Knotens ersetzt
                current.content = element;
                return;
            }
        }
        current.content = element;
    }

    /// Fügt ein neues Element an dem Index hinzu
    pub fn insert(&mut self, index: usize, element: T) {
        if index == 0 {
            self.add_first(element);
            return;
        }
        let mut current = match self.head.as_mut() {
            Some(node) => node,
            None => return, // Keine Einfügung wenn die Liste leer ist und index > 0
        };
        for _ in 0..(index - 1) {
            if let Some(next) = current.next.as_mut() {
                current = next;
            } else {
                return; // Index liegt außerhalb der Liste
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

    /// Entfernt das erste Element der Liste und gibt dessen Inhalt zurück.
    pub fn remove_first(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.content
        })
    }

    /// Entfernt das Element an dem Index
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

    /// Entfernt das erste Element was dem Inhalt entspricht und gibt diesen zurück
    pub fn remove(&mut self, element: &T) -> Option<T>
    where
        T: std::fmt::Display,
    {
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
        println!("Could not find node with the specified content");
        None
    }



    /// Guckt nach, ob das Element in der Liste drin ist
    pub fn get(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            if node.content == *element {
                return true;
            }
            current = node.next.as_ref();
        }
        false
    }
}