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

        // Nun sollte das älteste Element oben auf stack_out sein
        self.stack_out.pop()
    }

    // Gibt die Größe der Queue zurück
    pub fn size(&self) -> i32 {
        self.stack_in.size() + self.stack_out.size()
    }

    // Überprüft, ob die Queue leer ist
    pub fn is_empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }

    // Gibt die Queue als String zurück
    pub fn to_string(&self) -> String
    where
        T: ToString,
    {
        let mut result = String::new();
        let mut current = &self.stack_out.head;

        // Zuerst die Elemente von stack_out
        while let Some(node) = current {
            result.push_str(&node.data.to_string());
            if node.next.is_some() {
                result.push_str(" -> ");
            }
            current = &node.next;
        }

        // Dann die Elemente von stack_in (in umgekehrter Reihenfolge)
        let mut stack_in_vec = Vec::new();
        let mut current_in = &self.stack_in.head;
        while let Some(node) = current_in {
            stack_in_vec.push(node.data.to_string());
            current_in = &node.next;
        }

        if !stack_in_vec.is_empty() {
            if !result.is_empty() {
                result.push_str(" -> ");
            }
            result.push_str(&stack_in_vec.join(" -> "));
        }

        result
    }
}
