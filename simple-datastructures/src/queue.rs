use crate::datastructure::Datastructure;
use std::collections::VecDeque;

pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T: PartialEq + std::fmt::Display> Queue<T> {
    // Konstruktor: Erstellt eine neue Instanz des `CustomDeque`.
    pub fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }


    /// Fügt ein Element am Ende hinzu.
    pub fn enqueue_back(&mut self, value: T) {
        self.data.push_back(value);
    }

    // Fügt ein Element am Anfang hinzu.
    pub fn enqueue_front(&mut self, value: T) {
        self.data.push_front(value);
    }


    // Entfernt ein Element vom Anfang.
    pub fn dequeue_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    // Entfernt ein Element vom Ende.
    pub fn dequeue_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    // Gibt eine Referenz auf ein Element bei einem bestimmten Index zurück (nur lesend).
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }
}

    


// Implementierung des Datastructure-Traits für Queue
impl<T> Datastructure<T> for Queue<T> 
where 
    T: std::fmt::Display + ,
{

    // Gibt die Queue als String zurück
    fn to_string(&self) -> String {
        let mut result = String::from("[");
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.to_string());
        }
            
        result
    
    }

    

    // Überprüft, ob die Queue leer ist
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Gibt die Größe der Queue zurück
    fn size(&self) -> usize {
        self.data.len()
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
        queue.enqueue_front(10);
        queue.enqueue_front(20);
        queue.enqueue_front(30);

        let peek = queue.peek();
        assert_eq!(peek.unwrap(), &10);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();

        queue.enqueue_front(10);
        queue.enqueue_front(20);
        queue.enqueue_front(30);

        assert_eq!(queue.size(), 3);

        assert_eq!(queue.dequeue_back(), Some(10));
        assert_eq!(queue.dequeue_back(), Some(20));
        assert_eq!(queue.dequeue_back(), Some(30));

        assert!(queue.is_empty());
    }

    #[test]
    fn test_size_after_operations() {
        let mut queue = Queue::new();

        queue.enqueue_front(40);
        queue.enqueue_front(50);
        assert_eq!(queue.size(), 2);

        queue.dequeue_back();
        assert_eq!(queue.size(), 1);

        queue.dequeue_back();
        assert_eq!(queue.size(), 0);

        assert!(queue.is_empty());
    }

    #[test]
    fn test_to_string() {
        let mut queue = Queue::new();

        queue.enqueue_front(120);
        queue.enqueue_front(110);
        queue.enqueue_front(100);

        let output = queue.to_string();
        assert_eq!(output, "100 -> 110 -> 120");
    }

    #[test]
    fn test_equals() {
        let mut queue1 = Queue::new();
        let mut queue2 = Queue::new();

        queue1.enqueue_front(1);
        queue1.enqueue_front(2);
        queue2.enqueue_front(1);
        queue2.enqueue_front(2);

        assert!(queue1.equals(&queue2));

        queue2.enqueue_front(3);
        assert!(!queue1.equals(&queue2));
    }
}
