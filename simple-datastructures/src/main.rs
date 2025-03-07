mod stack;  // Modul deklarieren 
use stack::Stack;
mod queue;
use queue::Queue;
mod list;
use list::LinkedList;


fn main() {
    // **Stack Beispiel:**
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack (nach Pushen): {}", stack.to_string());

    if let Some(top) = stack.peek() {
        println!("Oberstes Element im Stack: {}", top);
    }

    if let Some(popped) = stack.pop() {
        println!("Pop vom Stack: {}", popped);
    }

    println!("Stack (nach Pop): {}", stack.to_string());
    println!("Stack Größe: {}", stack.size());
    println!("Ist der Stack leer? {}", stack.is_empty());

    // **LinkedList Beispiel:**
    let mut list = LinkedList::new();
    list.add(vec![10, 20, 30]);
    println!("LinkedList (nach Hinzufügen): {}", list.to_string());

    if let Some(value) = list.get(1) {
        println!("Element an Position 1 in der LinkedList: {}", value);
    }

    list.insert(25, 1);
    println!("LinkedList (nach Einfügen an Position 1): {}", list.to_string());

    list.remove(20);
    println!("LinkedList (nach Entfernen von 20): {}", list.to_string());

    list.replace(35, 0);
    println!("LinkedList (nach Ersetzen an Position 0): {}", list.to_string());

    println!("LinkedList Größe: {}", list.size());
    println!("Ist die LinkedList leer? {}", list.is_empty());

    // **Queue Beispiel:**
    let mut queue = Queue::new();
    queue.enqueue(5);
    queue.enqueue(10);
    queue.enqueue(15);
    
    println!("Queue (nach Enqueue): {}", queue.to_string());

    if let Some(front) = queue.peek() {
        println!("Vorderstes Element in der Queue: {}", front);
    }

    if let Some(dequeued) = queue.dequeue() {
        println!("Dequeue von der Queue: {}", dequeued);
    }

    println!("Queue (nach Dequeue): {}", queue.to_string());
    println!("Queue Größe: {}", queue.size());
    println!("Ist die Queue leer? {}", queue.is_empty());
}