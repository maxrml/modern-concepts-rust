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
    stack.push(4);
    stack.push(5);

    println!("Originaler Stack: {}", stack.to_string());

    println!("Stack mit lazy_map verdoppelt:");
    let mut mapped_stack = stack.lazy_map(|x| x * 2);
    while let Some(val) = mapped_stack.next() {
        print!("{} ", val);
    }
    println!();

    println!("Stack mit lazy_filter (nur gerade Zahlen):");
    let mut filtered_stack = stack.lazy_filter(|x| x % 2 == 0);
    while let Some(val) = filtered_stack.next() {
        print!("{} ", val);
    }
    println!();

    let mut queue = Queue::new();
    queue.enqueue(10);
    queue.enqueue(15);
    queue.enqueue(20);
    queue.enqueue(25);
    queue.enqueue(30);

    println!("\nOriginale Queue: {:?}", queue.to_string());

    println!("Queue mit lazy_map quadriert:");
    let mut mapped_queue = queue.lazy_map(|x| x * x);
    while let Some(val) = mapped_queue.next() {
        print!("{} ", val);
    }
    println!();

    println!("Queue mit lazy_filter (nur > 20):");
    let mut filtered_queue = queue.lazy_filter(|&x| x > 20);
    while let Some(val) = filtered_queue.next() {
        print!("{} ", val);
    }
    println!();

    let mut list = LinkedListDS::new();
    list.push_back(100);
    list.push_back(200);
    list.push_back(300);
    list.push_back(400);
    list.push_back(500);

    println!("\nOriginale LinkedList: {}", list.to_string());

    println!("LinkedList mit lazy_map halbiert:");
    let mut mapped_list = list.lazy_map(|x| x / 2);
    while let Some(val) = mapped_list.next() {
        print!("{} ", val);
    }
    println!();

    println!("LinkedList mit lazy_filter (>= 300):");
    let mut filtered_list = list.lazy_filter(|&x| x >= 300);
    while let Some(val) = filtered_list.next() {
        print!("{} ", val);
    }
    println!();
}
