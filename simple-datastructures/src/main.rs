mod stack;
use stack::Stack;
mod queue;
use queue::Queue;
mod list;
use list::LinkedList;
mod datastructure;
use crate::datastructure::Datastructure;

fn main() {
    println!("===================================");
    println!("Stack");
    println!("===================================");
    // Demonstration der Stack-Klasse
    let mut stack: Stack<i32> = Stack::new();

    // Push-Operationen
    stack.push(10);
    stack.push(20);
    stack.push(30);

    // Ausgabe des Stacks
    println!("Stack: {}", stack.to_string());

    // Pop-Operation
    if let Some(val) = stack.pop() {
        println!("Pop: {}", val);
    }

    // Stack nach Pop
    println!("Stack nach Pop: {}", stack.to_string());

    // Stack-Methoden
    println!("Stack Größe: {}", stack.size());
    println!("Ist der Stack leer? {}", stack.is_empty());
    println!("Ist der Stack voll? {}", stack.is_full());
    println!("Was ist das oberste Element? {:?}", stack.peek());

    // Erstellen eines weiteren Stacks und vergleichen
    let mut another_stack = Stack::new();
    another_stack.push(20);
    another_stack.push(10);

    println!("Stack zu vergleichen: {}", stack.to_string());
    println!("another_stack nach Pop: {}", another_stack.to_string());

    println!("Sind die beiden Stacks gleich? {}", stack.equals(&another_stack));

    // Mehrere Elemente auf Stack pushen
    let mut more_stack: Stack<i32> = Stack::new();
    more_stack.push_all(vec![1, 2, 3, 4, 5]);
    println!("Oberster Wert im Stack: {}", more_stack.peek().unwrap());
    println!("Vector Stack: {}", stack.to_string());

    // komplexe Datentypen Array
    let a = [1, 2, 3, 4, 5];
    let b = [1, 32, 4235, 2324];
    let c = [1, 32, 4235, 2324, 2, 3, 4, 5];

    let mut stack_complex: Stack<&[i32]> = Stack::new();
    stack_complex.push(&a);
    stack_complex.push(&b);
    stack_complex.push(&c);

    println!("Komplexe Datentype im Stack: {:#?}", stack_complex.peek().unwrap());
    stack_complex.pop();
    stack_complex.pop();
    stack_complex.pop();

    println!("===================================");
    println!("Queue");
    println!("===================================");

    let mut queue = Queue::new();
    queue.enqueue_front(1);
    queue.enqueue_back(2);
    queue.enqueue_front(0);

    queue.is_empty();
    queue.is_full();
    queue.size();
    queue.dequeue_back();
    queue.dequeue_front();

    // Aufruf der benutzerdefinierten to_string Methode
    println!("{}", queue.to_string()); // Gibt "[0, 1, 2]" aus

    println!("===================================");
    println!("Linked List Tests");
    println!("===================================");

    let mut list: LinkedList<i32> = LinkedList::new();
    println!("is_empty erster Aufruf: {}", list.is_empty());
    println!("is_empty zweiter Aufruf: {}", list.is_empty());

    // Test: add_first (2-mal)
    list.add_first(10);
    println!("Nach add_first(10): {}", list.to_string());
    list.add_first(20);
    println!("Nach add_first(20): {}", list.to_string());

    // Test: add (2-mal)
    list.add(30);
    println!("Nach add(30): {}", list.to_string());
    list.add(40);
    println!("Nach add(40): {}", list.to_string());

    // Test: size (2-mal)
    println!("size erster Aufruf: {}", list.size());
    println!("size zweiter Aufruf: {}", list.size());

    // Test: content (2-mal; hier für Index 0 und 1)
    println!("content an Index 0: {:?}", list.content(0).unwrap());
    println!("content an Index 1: {:?}", list.content(1).unwrap());

    // Test: replace (2-mal)
    list.replace(1, 15);
    println!("Nach replace(1, 15): {}", list.to_string());
    list.replace(1, 25);
    println!("Nach replace(1, 25): {}", list.to_string());

    // Test: insert (2-mal; Einfügen an Index 2)
    list.insert(2, 35);
    println!("Nach insert(2, 35): {}", list.to_string());
    list.insert(2, 45);
    println!("Nach insert(2, 45): {}", list.to_string());

    // Test: remove_first (2-mal)
    if let Some(removed) = list.remove_first() {
        println!("remove_first erster Aufruf, entfernt: {}", removed);
        println!("Nach remove_first: {}", list.to_string());
    }
    if let Some(removed) = list.remove_first() {
        println!("remove_first zweiter Aufruf, entfernt: {}", removed);
        println!("Nach remove_first: {}", list.to_string());
    }

    // Test: remove_at (2-mal; entferne Element an Index 1)
    list.remove_at(1);
    println!("Nach remove_at(1) erster Aufruf: {}", list.to_string());
    list.remove_at(1);
    println!("Nach remove_at(1) zweiter Aufruf: {}", list.to_string());

    // Test: remove (element) (2-mal)
    // Damit wir Elemente zum Entfernen haben, fügen wir neue hinzu:
    list.add(50);
    list.add(60);
    println!("Vor remove(element): {}", list.to_string());
    if let Some(removed) = list.remove(&50) {
        println!("remove(element) erster Aufruf, entfernt: {}", removed);
        println!("Nach remove(50): {}", list.to_string());
    }
    if let Some(removed) = list.remove(&60) {
        println!("remove(element) zweiter Aufruf, entfernt: {}", removed);
        println!("Nach remove(60): {}", list.to_string());
    }

    // Test: get (2-mal)
    println!("get(25) erster Aufruf: {}", list.get(&25));
    println!("get(45) zweiter Aufruf: {}", list.get(&45));

    // For no unused warnings
    queue.size();
    stack.peek();
}
