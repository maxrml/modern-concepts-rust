// Library for handling user inputs
use std::io;

// Class declaration
struct RPNCalculator {
    stack: Vec<f64>,
    latex_stack: Vec<String>,
}

impl RPNCalculator {
    // Constructor, giving the objects a stack history (infix and latex)
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            latex_stack: Vec::new(),
        }
    }

    // Applying operation the the stack
    fn apply_operation() {

    }

    // Pop to history (infix)
    fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }

    // Pop to history (latex)
    fn pop_latex(&mut self) -> Option<String> {
        self.latex_stack.pop()
    }

    fn welcome_prompt() {
        println!("-----------------------------");
        println!("Welcome to the RPN calculator, please input your equation with 'enter' between!");
        println!("-----------------------------");
    }

}


// Main function for executing the RPN calculator
fn main() {
    let mut calc = RPNCalculator::new();
    RPNCalculator::welcome_prompt();
    let mut input = String::new();
    // "Main loop", repeating logic for each input
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
    }

}
