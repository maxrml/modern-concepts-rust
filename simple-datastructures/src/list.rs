use crate::stack::Stack;
use crate::stack::Node;
#[derive(Clone)]
pub struct LinkedList<T> {
    stack: Stack<T>,    // Wir nutzen den Stack als Baustein
    tail: Option<Box<Node<T>>>,  // Der `tail`-Zeiger auf das letzte Element
}

impl<T> LinkedList<T>
where T: Clone,
{
    pub fn new() -> Self {
        LinkedList {
            stack: Stack::new(), // Der Stack wird hier als Baustein verwendet
            tail: None,          // Anfangs gibt es keinen Tail
        }
    }

    // Fügt ein Element am Anfang der Liste hinzu
    pub fn push_front(&mut self, data: T) {
        self.stack.push(data);

        // Wenn die Liste leer war, ist das neue Element sowohl `head` als auch `tail`
        if self.tail.is_none() {
            if let Some(ref mut head) = self.stack.head {
                self.tail = Some(head.clone());
            }
        }
    }

    // Fügt ein Element am Ende der Liste hinzu
    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data.clone(),
            next: None,
        });

        // Wenn die Liste leer ist, setzen wir das neue Element als `head` und `tail`
        if self.stack.is_empty() {
            self.stack.push(data); // Ein Element als `head`
            self.tail = Some(new_node); // Setzen des `tail`
        } else {
            // Andernfalls fügen wir das neue Element hinter dem aktuellen `tail` hinzu
            if let Some(ref mut tail) = self.tail {
                tail.next = Some(new_node.clone());
            }
            self.tail = Some(new_node); // Das neue Element wird zum `tail`
        }
    }

    // Entfernt das erste Element der Liste
    pub fn pop_front(&mut self) -> Option<T> {
        let popped = self.stack.pop();
        
        // Wenn die Liste jetzt leer ist, setzen wir `tail` auf `None`
        if self.stack.is_empty() {
            self.tail = None;
        }
        
        popped
    }

    // Gibt die Größe der Liste zurück
    pub fn size(&self) -> i32 {
        self.stack.size()
    }

    // Überprüft, ob die Liste leer ist
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    // Gibt die Liste als String zurück
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        self.stack.to_string()
    }

    // Gibt das letzte Element der Liste zurück
    pub fn peek_tail(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.data)
    }
}

