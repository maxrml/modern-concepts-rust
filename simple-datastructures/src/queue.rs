use crate::stack::Stack;
use crate::datastructure::Datastructure;

pub struct Queue<T> {
    stack_in: Stack<T>,  // Stack für das Einfügen von Elementen
    stack_out: Stack<T>, // Stack für das Entfernen von Elementen
}

impl<T: PartialEq + std::fmt::Display> Queue<T> {
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
        if Datastructure::is_empty(&self.stack_out) {
            while let Some(data) = self.stack_in.pop() {
                self.stack_out.push(data);
            }
        }

        // Nun sollte das älteste Element oben auf stack_out sein
        self.stack_out.pop()
    }

        
}

    


// Implementierung des Datastructure-Traits für Queue
impl<T> Datastructure<T> for Queue<T> 
where 
    T: PartialEq + ToString + std::fmt::Display + ,
{

    // Gibt die Queue als String zurück
    fn to_string(&self) -> String {
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

    fn peek(&mut self) -> Option<&T> {
        // Falls stack_out leer ist, müssen wir zuerst die Elemente umschichten
        if self.stack_out.is_empty() {
            while let Some(data) = self.stack_in.pop() {
                self.stack_out.push(data);
            }   
        }
        self.stack_out.peek()  
    }

    // Überprüft, ob die Queue leer ist
    fn is_empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }

    // Gibt die Größe der Queue zurück
    fn size(&self) -> i32 {
        self.stack_in.size() + self.stack_out.size()
    }

    
} 

// ------------------------------Testing--------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_peek() {
        let mut queue: Queue<i32> = Queue::new();
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        let peek = queue.peek();
        assert_eq!(peek.unwrap(), &10);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.size(), 3);
        
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        
        assert!(queue.is_empty());
    }

    #[test]
    fn test_size_after_operations() {
        let mut queue = Queue::new();
        
        queue.enqueue(40);
        queue.enqueue(50);
        assert_eq!(queue.size(), 2);
        
        queue.dequeue();
        assert_eq!(queue.size(), 1);
        
        queue.dequeue();
        assert_eq!(queue.size(), 0);

        assert!(queue.is_empty());
    }

    #[test]
    fn test_to_string() {
        let mut queue = Queue::new();
        
        queue.enqueue(120);
        queue.enqueue(110);
        queue.enqueue(100);

        let output = queue.to_string();
        assert_eq!(output, "100 -> 110 -> 120");
    }

    #[test]
    fn test_equals() {
        let mut queue1 = Queue::new();
        let mut queue2 = Queue::new();
        
        queue1.enqueue(1);
        queue1.enqueue(2);
        queue2.enqueue(1);
        queue2.enqueue(2);
        
        assert!(queue1.equals(&queue2));
        
        queue2.enqueue(3);
        assert!(!queue1.equals(&queue2));
    }
}
