// Library for handling user inputs
use std::env;

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

}


// Main function for executing the RPN calculator
fn main() {
    let mut calc = RPNCalculator::new();
    // "Main loop", repeating logic for each input
    loop {
    }

}
