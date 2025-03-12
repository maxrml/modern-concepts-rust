mod datastructures;
use crate::datastructures::Datastructure;
mod queue;
use std::collections::VecDeque;
use queue::Queue;
mod list;
use list::LinkedListDS;
use std::collections::LinkedList;
mod stack;
use std::vec::Vec;
use stack::Stack;


fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Map-Funktion: Multipliziert jedes Element mit 2
    let doubled_stack = stack.map(|x| x * 2, Stack::new());

    println!("Originaler Stack: {}", stack.to_string());
    println!("Verdoppelter Stack: {}", doubled_stack.to_string());

    // Beispiel mit Queue
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    let incremented_queue = queue.map(|x| x + 1, Queue::new());

    println!("Originale Queue: {}", queue.to_string());
    println!("Inkrementierte Queue: {}", incremented_queue.to_string());

    // Beispiel mit LinkedListDS
    let mut list = LinkedListDS::new();
    list.push_back(5);
    list.push_back(15);
    list.push_back(25);

    let mut squared_list = list.map(|x| x * x, Queue::new());

    squared_list.dequeue();

    println!("Originale Liste: {}", list.to_string());
    println!("Quadrierte Liste: {}", squared_list.to_string());

}