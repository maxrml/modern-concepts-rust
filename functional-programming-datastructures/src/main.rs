mod datastructures;
use crate::datastructures::Datastructure;
mod queue;
use std::collections::VecDeque;
use queue::Queue;
mod list;


fn main() {
    println!("===================================");
        println!("Queue");
        println!("===================================");

        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        println!("Queue: {}", queue.to_string()); // Output: ältestes 3 -> 2 -> 1 neuste

        println!("Dequeue: {:?}", queue.dequeue().unwrap()); // Output: Some(1)
        println!("Queue: {}", queue.to_string()); // Output: neuste 2 -> 3 älteste weil stack in stack out

        queue.enqueue(4);
        println!("Queue: {}", queue.to_string()); // Output: 2 -> 3 -> 4

}