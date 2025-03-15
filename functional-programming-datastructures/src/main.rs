mod datastructures;
use crate::datastructures::Datastructure;
mod queue;
use queue::Queue;
mod list;
use list::List;
mod stack;
use stack::Stack;
mod lazy_evaluation;
mod order;



fn main() {
    // Test Queue
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), None);

    // Test Datastructure-Trait für Queue
    let mut queue = Queue::new();
    queue.insert(1);
    queue.insert(2);
    queue.insert(3);
    
    assert_eq!(queue.to_string(), "[1, 2, 3]");
    assert_eq!(queue.size(), 3);
    assert!(!queue.is_empty());

    // Test List
    let mut list = List::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_front(), None);

    // Test Datastructure-Trait für List
    let mut list = List::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    
    assert_eq!(list.to_string(), "[1, 2, 3]");
    assert_eq!(list.size(), 3);
    assert!(!list.is_empty());

    // Map-Operation für List
    let target = List::new();
    let mapped = list.map(|x| x * 2, target);
    assert_eq!(mapped.to_string(), "[2, 4, 6]");

    // Filter-Operation für List
    let target = List::new();
    let filtered = list.filter(|x| x % 2 == 0, target);
    assert_eq!(filtered.to_string(), "[2]");

    // Reduce-Operation für List
    let sum = list.reduce(|acc, x| acc + x, 0);
    assert_eq!(sum, 6);

    // Lazy-Operationen für List
    let mut lazy_mapped = list.lazy_map(|x| x * 2);
    let mut result = List::new();
    while let Some(x) = lazy_mapped.next_map() {
        result.insert(x);
    }
    assert_eq!(result.to_string(), "[2, 4, 6]");

    let mut lazy_filtered = list.lazy_filter(|x| x % 2 != 0);
    let mut result = List::new();
    while let Some(x) = lazy_filtered.next_filter() {
        result.insert(x);
    }
    assert_eq!(result.to_string(), "[1, 3]");

    // Test Stack
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);

    // Test Datastructure-Trait für Stack
    let mut stack = Stack::new();
    stack.insert(1);
    stack.insert(2);
    stack.insert(3);
    
    assert_eq!(stack.to_string(), "[1, 2, 3]");
    assert_eq!(stack.size(), 3);
    assert!(!stack.is_empty());

    // Map-Operation für Stack
    let target = Stack::new();
    let mapped = stack.map(|x| x * x, target);
    assert_eq!(mapped.to_string(), "[1, 4, 9]");

    // Filter-Operation für Stack
    let target = Stack::new();
    let filtered = stack.filter(|x| x % 2 != 0, target);
    assert_eq!(filtered.to_string(), "[1, 3]");

    // Reduce-Operation für Stack
    let product = stack.reduce(|acc, x| acc * x, 1);
    assert_eq!(product, 6);

    // Lazy-Operationen für Stack
    let mut lazy_mapped = stack.lazy_map(|x| x * 2);
    let mut result = Stack::new();
    while let Some(x) = lazy_mapped.next_map() {
        result.insert(x);
    }
    assert_eq!(result.to_string(), "[6, 4, 2]");

    let mut lazy_filtered = stack.lazy_filter(|x| x > &1);
    let mut result = Stack::new();
    while let Some(x) = lazy_filtered.next_filter() {
        result.insert(x);
    }
    assert_eq!(result.to_string(), "[3, 2]");

    println!("Alle Tests erfolgreich!");
}
