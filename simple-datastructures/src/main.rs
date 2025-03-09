

mod stack;
use stack::Stack;


fn main() {
    // **Stack Beispiel:**
let mut stack: Stack<i32> = Stack::new();
stack.push(1);
stack.push(2);
stack.push(3);



stack.peek();

stack.pop();




println!("{}",stack);

    //  methods are not working
}