use crate::stack::Stack;
pub struct Queue<T> {
    stack_in: Stack<T>,  // Stack für das Einfügen von Elementen
    stack_out: Stack<T>, // Stack für das Entfernen von Elementen
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            stack_in: Stack::new(),
            stack_out: Stack::new(),
        }
    }

    // Fügt ein Element in die Queue ein (Push auf stack_in)
    pub fn enqueue(&mut self, data: T) {
        self.stack_in.push(data);
    }

    // Entfernt das älteste Element aus der Queue (Pop von stack_out)
    pub fn dequeue(&mut self) -> Option<T> {
        // Wenn stack_out leer ist, verschiebe alle Elemente von stack_in nach stack_out
        if self.stack_out.is_empty() {
            while let Some(data) = self.stack_in.pop() {
                self.stack_out.push(data);
            }
        }

        self.stack_out.pop()
    }

    pub fn peek(&mut self) -> Option<&T> {
        // Falls stack_out leer ist, müssen wir zuerst die Elemente umschichten
        if self.stack_out.is_empty() {
            while let Some(data) = self.stack_in.pop() {
                self.stack_out.push(data);
            }
        }

        // Jetzt enthält stack_out das älteste Element oben
        self.stack_out.peek()
    }

    // Gibt die Größe der Queue zurück
    pub fn size(&self) -> i32 {
        self.stack_in.size() + self.stack_out.size()
    }

    // Überprüft, ob die Queue leer ist
    pub fn is_empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }

    pub fn is_full(&self) -> bool {
        !self.stack_in.is_empty() || !self.stack_out.is_empty()
    }

    // Gibt die Queue als String zurück
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        let mut elements = Vec::new();
        let mut stack_out_vec = Vec::new();
        let mut current = &self.stack_out.head;

        // Zuerst die Elemente von stack_out
        while let Some(node) = current {
            stack_out_vec.push(node.data.to_string());
            current = &node.next;
        }
        stack_out_vec.reverse(); // Reihenfolge korrigieren

        // Dann die Elemente von stack_in (richtige Reihenfolge)
        let mut current_in = &self.stack_in.head;
        while let Some(node) = current_in {
            elements.push(node.data.to_string());
            current_in = &node.next;
        }

        stack_out_vec.extend(elements);
        stack_out_vec.join(" -> ")
    }
}
