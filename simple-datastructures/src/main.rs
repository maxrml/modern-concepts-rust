mod stack;  // Modul deklarieren 
use stack::Stack; // die Struktur aus Modul importieren

mod list;  // Modul deklarieren 
use list::LinkedList; // die Struktur aus Modul importieren

fn main() {

    // **********STACK**********


    let mut stack = Stack::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push_all(vec![2, 3, 4, 5]);

    println!("Top of stack: {:?}", stack.peek());
    println!("Stack full: {}", stack.is_full());
    
    

    while !stack.is_empty() {  
        stack.peek();
        println!("Popped: {:?}", stack.pop());
    }

    match stack.pop() {
        Ok(value) => println!("Popped value: {}", value),
        Err(e) => println!("Error: {:?}", e),
    }

    
    // **********LINKED LIST**********



    let mut list = LinkedList::new();

    
    match list.add(vec![1, 2, 3]) {
        Ok(_) => println!("Elements added."),
        Err(e) => println!("Error adding elements: {:?}", e),
    }

    match list.insert(10, 1) {
        Ok(_) => println!("{:?}, size: {}", list, list.size()),
        Err(e) => println!("Error inserting element: {:?}", e),
    }
    
    match list.remove(2) {
        Ok(_) => println!("Removed element 2."),
        Err(e) => println!("Error removing element: {:?}", e),
    }
    
    match list.remove_at(0) {
        Ok(_) => println!("Removed element at position 0."),
        Err(e) => println!("Error removing element at position: {:?}", e),
    }
    
    match list.replace(20, 0) {
        Ok(_) => println!("Replaced element at position 0."),
        Err(e) => println!("Error replacing element: {:?}", e),
    }
    
    match list.get(0) {
        Ok(value) => println!("Element at position 0: {:?}", value),
        Err(e) => println!("Error retrieving element: {:?}", e),
    }
    
    println!("Is list empty? {}", list.is_empty());
    println!("Is list full? {}", list.is_full());
}