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

    // Fügt mehrere Elemente oben auf den Stack
    pub fn push_all<I: IntoIterator<Item = T>>(&mut self, data: I) {
        for item in data {
            self.push(item);
        }
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
impl<T> Datastructure<T> for Stack<T>
where
    T: PartialEq + ToString + std::fmt::Display,
{
    

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

    // Gibt das oberste Element zurück, ohne es zu entfernen
    fn peek(&mut self) -> Option<&T>{
        match self.head {
            Some(ref node) => Some(&node.data),
            None => None,// Wenn der Stack leer ist, geben wir None zurück
        }
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

// ------------------------------Testing--------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.size(), 2);
        assert_eq!(stack.peek(), Some(&20));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(30);
        stack.push(40);
        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(50);
        assert_eq!(stack.peek(), Some(&50));

        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_to_string() {
        let mut stack = Stack::new();
        stack.push(60);
        stack.push(70);
        stack.push(80);
        assert_eq!(stack.to_string(), "80 -> 70 -> 60");
    }

    #[test]
    fn test_equals() {
        let mut stack1 = Stack::new();
        let mut stack2 = Stack::new();

        stack1.push(90);
        stack1.push(100);
        stack2.push(90);
        stack2.push(100);

        assert!(stack1.equals(&stack2));

        stack2.pop();
        assert!(!stack1.equals(&stack2));
    }
}