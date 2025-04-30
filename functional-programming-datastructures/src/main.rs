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

    let mut my_stack = Stack::new();
    let mut my_queue = Queue::new();

    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(4);
    my_stack.push(5);
    my_stack.push(8);
    my_stack.push(16);


    //Closures
    let square = |&x: &i32| x * x;
    let plus_one = |&x: &i32| x + 1;
    let is_even = |x: &i32| x % 2 == 0;
    let multiply = |x: &i32| x * 3;

    //map-Funktion
    let my_list = my_stack.map(square, LinkedListDS::new());
    println!("Originaler Stack (my_stack): {}", my_stack.to_string());
    println!("Quadrierter Stack als Liste (my_list): {}\n", my_list.to_string());

    //Mut Closure
    let mut counter = 0;
    let mut plus_counter = |&x: &i32| { 
        counter += 1; 
        x + counter 
    };

    let mut new_list = my_list.map(&mut plus_counter, LinkedListDS::new());
        println!("Added Counter to list_values: {}\n", new_list.to_string());

    //Verkettung
    let mut new_queue = new_list.map(plus_one, Stack::new()).map(multiply, Queue::new()).filter(is_even, my_queue);
    println!("Neue Queue nach verkettetem Aufruf: {}\n", new_queue.to_string());
    new_queue.enqueue(5);
    new_queue.enqueue(12);
    new_queue.enqueue(7);
    
    //Neuen Stack erstellen und Reduce-Funktion
    let mut stack = new_queue.map(plus_one, Stack::new());
    println!("Neuer Stack: {}", stack.to_string());
    let sum = stack.reduce(|acc, &x| acc + x, 0);
    println!("Summe (reduce): {}", sum);

    //Neue Liste erstellen mithilfe von for_each:
    let mut list = LinkedListDS::new();
    stack.for_each(|&x| list.push_back(x));
    println!("Die kopierte Liste: {}", list.to_string());
    let sum = list.reduce(|acc, &x| acc + x, 0);
    println!("Summe der Liste (gleicher Wert): {} \n", sum);




    //Lazy Funtionen
    println!("Datenstruktur die für Lazy verwendet wird:{}", stack.to_string());
    let doubled: Vec<_> = stack.lazy_map(|&x| x * 2).collect();
    println!("Verdoppelte Werte (lazy_map): {:?}", doubled);
    

    let filtered: Vec<_> = stack.lazy_filter(|x| *x == 13).collect();
    println!("Gefilterte Werte (nur 13, lazy_filter): {:?}\n", filtered);

    //Beispiel für Strings:
    let mut stack = Stack::new();

    // Strings in den Stack pushen
    stack.push("Hallo");
    stack.push("Rust");
    stack.push("Hallo");
    stack.push("Welt");

    // Demonstration von for_each
    let mut string_queue = Queue::new();
    println!("Alle Elemente im Stack:");
    stack.for_each(|x| {
            string_queue.enqueue(x.to_uppercase());
    });
    println!("Queue mit Großbuchstaben: {}", string_queue.to_string());

    // Demonstration von reduce (verkettete Strings erzeugen)
    let concatenated = stack.reduce(|acc, x| acc + "+" + x, String::new());
    println!("Verkettete Strings (reduce): {}", concatenated.trim());
    
}

