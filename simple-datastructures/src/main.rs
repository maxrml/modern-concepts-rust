mod stack;  // Modul deklarieren 
use stack::Stack;
mod queue;
use queue::Queue;
mod list;
use list::LinkedList;


fn main() {
    // Demonstration der Stack-Klasse
    println!("--- Stack Demo ---");
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

    // Erstellen eines weiteren Stacks und vergleichen
    let mut another_stack = Stack::new();
    another_stack.push(20);
    another_stack.push(10);
    
    println!("Sind die beiden Stacks gleich? {}", stack.equals(&another_stack));

    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    println!("Queue: {}", queue.to_string()); // Output: 1 -> 2 -> 3

    println!("Dequeue: {:?}", queue.dequeue()); // Output: Some(1)
    println!("Queue: {}", queue.to_string()); // Output: 2 -> 3

    queue.enqueue(4);
    println!("Queue: {}", queue.to_string()); // Output: 2 -> 3 -> 4
    
    let mut list = LinkedList::new();

    // Füge einige Elemente hinzu
    println!("Füge 10 hinzu:");
    list.push_back(10);
    println!("{}", list.to_string());  // Erwartete Ausgabe: 10

    println!("Füge 20 hinzu:");
    list.push_back(20);
    println!("{}", list.to_string());  // Erwartete Ausgabe: 10 -> 20

    println!("Füge 30 hinzu:");
    list.push_back(30);
    println!("{}", list.to_string());  // Erwartete Ausgabe: 10 -> 20 -> 30

    // Füge ein Element vorne hinzu
    println!("Füge 5 vorne hinzu:");
    list.push_front(5);
    println!("{}", list.to_string());  // Erwartete Ausgabe: 5 -> 10 -> 20 -> 30

    // Poppe das erste Element (vorne)
    println!("Poppe das erste Element (vorne):");
    list.pop_front();
    println!("{}", list.to_string());  // Erwartete Ausgabe: 10 -> 20 -> 30

    // Zeige das letzte Element (tail)
    println!("Das letzte Element ist: {:?}", list.peek_tail());  // Erwartete Ausgabe: Some(30)

    // Teste die Größe der Liste
    println!("Die Größe der Liste ist: {}", list.size());  // Erwartete Ausgabe: 3

    // Überprüfe, ob die Liste leer ist
    println!("Ist die Liste leer? {}", list.is_empty());  // Erwartete Ausgabe: false

    // Entferne alle Elemente
    println!("Poppe alle Elemente:");
    while let Some(item) = list.pop_front() {
        println!("Entferntes Element: {}", item);
    }
    println!("Ist die Liste leer? {}", list.is_empty());  

    
}
