use std::collections::VecDeque;
use crate::datastructure::Datastructure;


pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            data: VecDeque::new(),
        }
    }

    pub fn enque(&mut self, value: T) {
        self.data.push_back(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }  
}


// Implementierung des Datastructure-Traits für Queue
impl<T> Datastructure<T> for Queue<T> where T: PartialEq + ToString + std::fmt::Display {
    // Gibt die Queue als String zurück
    
    fn to_string(&self) -> String 
    where T: std::fmt::Display {
        let mut result = String::from("[");
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&item.to_string());
        }
        result.push(']');
        result
    }

    // Überprüft, ob die Queue leer ist
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // Gibt die Größe der Queue zurück
    fn size(&self) -> i32 {
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
