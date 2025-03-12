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
    
    println!("==== Stack Tests ====");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    
    // Test reduce (Summe)
    let sum_stack: i32 = stack.reduce(|acc, &x| acc + x, 0);
    println!("Summe des Stacks: {}", sum_stack);

    // Test map (Quadrieren der Elemente)
    let squared_stack: Stack<i32> = stack.map(|x| x * x, Stack::new());
    println!("Quadratisierter Stack: {}", squared_stack.to_string());

    // Test forEach (Einzelnes Printen)
    println!("Stack Elemente:");
    stack.for_each(|x| println!("{}", x));

    // Test filter (Nur gerade Zahlen)
    let even_stack: Stack<i32> = stack.filter(|&x| x % 2 == 0, Stack::new());
    println!("Gerade Zahlen im Stack: {}", even_stack.to_string());

    println!("\n==== Queue Tests ====");
    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.enqueue(40);

    // Test reduce (Multiplikation)
    let product_queue: i32 = queue.reduce(|acc, &x| acc * x, 1);
    println!("Produkt der Queue: {}", product_queue);

    // Test map (Halbieren der Werte)
    let half_queue: Queue<i32> = queue.map(|x| x / 2, Queue::new());
    println!("Halbierte Queue: {}", half_queue.to_string());

    // Test forEach (Printen der Queue)
    println!("Queue Elemente:");
    queue.for_each(|x| println!("{}", x));

    // Test filter (Nur Werte größer als 15)
    let filtered_queue: Queue<i32> = queue.filter(|&x| x > 15, Queue::new());
    println!("Gefilterte Queue (x > 15): {}", filtered_queue.to_string());

    println!("\n==== LinkedList Tests ====");
    let mut list = LinkedListDS::new();
    list.push_back("Apfel".to_string());
    list.push_back("Banane".to_string());
    list.push_back("Kiwi".to_string());

    // Test reduce (Konkatenation der Strings)
    let concat_list: String = list.reduce(|acc, x| acc + " " + x, String::new());
    println!("Konkatenierte LinkedList: {}", concat_list.trim());

    // Test map (Strings in Großbuchstaben)
    let uppercase_list: LinkedListDS<String> = list.map(|x| x.to_uppercase(), LinkedListDS::new());
    println!("Großbuchstaben LinkedList: {}", uppercase_list.to_string());

    // Test forEach (Einzelnes Printen)
    println!("LinkedList Elemente:");
    list.for_each(|x| println!("{}", x));

    // Test filter (Nur Strings mit mehr als 5 Buchstaben)
    let long_words: LinkedListDS<String> = list.filter(|x| x.len() > 5, LinkedListDS::new());
    println!("Lange Wörter in LinkedList: {}", long_words.to_string());

}