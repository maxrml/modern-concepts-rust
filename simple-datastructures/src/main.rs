mod stack;
use stack::Stack;




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

    
}