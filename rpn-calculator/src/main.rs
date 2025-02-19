// Library for handling user inputs
use std::io;

// Class declaration
struct RPNCalculator {
    stack: Vec<f64>,
    latex_stack_history: String,
    stack_history: String,
    indicator: bool,
}

impl RPNCalculator {
    // Constructor, initializing the vectors
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            latex_stack_history: String::new(),
            stack_history: String::new(),
            indicator: true,
        }
    }

    // Applying operation to the stack
    fn apply_operation(&mut self, token: &str) {
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                self.arithmetical_operation_handling(token);
            }
            "sqrt" | "log" | "abs" => {
                self.log_abs_sqrt_operation_handling(token);
            }
            "!" => {
                self.factorial_operation_handling();
            }
            "++" => {
                self.full_stack_addition_handling();
            }
            "**" => {
                self.full_stack_multiplication_handling();
            }
            _ => {
                self.new_number_handling(token);
            }
        }
    }

    fn arithmetical_operation_handling(&mut self, token: &str) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match token {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "^" => a.powf(b),
            _ => unreachable!(),
        };

            self.stack_history = format!("({}) {} {}", self.stack_history, token, b);
            match token {
                "+" => {
                    self.latex_stack_history = format!(
                        "{{{}}} {} {}",
                        self.latex_stack_history,
                        token,
                        b
                    );
                }
                "-" => {
                    self.latex_stack_history = format!(
                        "{{{}}} {} {}",
                        self.latex_stack_history,
                        token,
                        b
                    );
                }
                "*" => {
                    self.latex_stack_history = format!(
                        r"{{{}}} \cdot {}",
                        self.latex_stack_history,
                        b
                    );
                }
                "/" => {
                    self.latex_stack_history = format!(
                        r"\frac {{{}}} {{{}}}",
                        self.latex_stack_history,
                        b
                    );
                }
                "^" => {
                    self.latex_stack_history = format!(
                        r"{{{}}}^{{{}}}",
                        self.latex_stack_history,
                        b
                    );
                }
                _ => panic!(),
            }
            self.stack.push(result);
        
    }

    fn log_abs_sqrt_operation_handling(&mut self, token: &str) {
        let a = self.stack.pop().unwrap();
        let result = match token {
            "sqrt" => a.sqrt(),
            "log" => a.log10(),
            "abs" => a.abs(),
            _ => unreachable!(),
        };

        match token {
            "sqrt" => {
                self.stack_history = format!(r"(sqrt({}))", self.stack_history);
                self.latex_stack_history = format!(r"\sqrt{{{}}}", self.latex_stack_history);
            }
            "log" => {
                self.stack_history = format!(r"(log({}))", self.stack_history);
                self.latex_stack_history = format!(r"\log_{{10}} {{{}}}", self.latex_stack_history);
            }
            "abs" => {
                self.stack_history = format!("|{}|", self.stack_history);
                self.latex_stack_history = format!(
                    r"\left| {{{}}} \right|",
                    self.latex_stack_history
                );
            }
            _ => panic!(),
        }

        self.stack.push(result);
    }

    fn factorial_operation_handling(&mut self) {
        let a = self.stack.pop().unwrap();
        let result = (1..=a as u64).product::<u64>() as f64;

        self.stack_history = format!("{{{}}}!", self.stack_history);
        self.latex_stack_history = format!("{{{}}}!", self.latex_stack_history);

        self.stack.push(result);
    }

    fn full_stack_addition_handling(&mut self) {
        let result: f64 = self.stack.iter().sum();
    
        // Save the first number separately
        if let Some(&first) = self.stack.first() {
            self.latex_stack_history = format!("{}", first);
            self.stack_history = format!("{}", first);
            
            // Start from the second number
            for &num in self.stack.iter().skip(1) {
                self.latex_stack_history = format!(r"{{{}}} + {}", self.latex_stack_history, num);
                self.stack_history = format!("({}) + {}", self.stack_history, num);
            }
        }
    
        self.stack.clear();
        self.stack.push(result);
    }
    
    fn full_stack_multiplication_handling(&mut self) {
        let result: f64 = self.stack.iter().product();
    
        // Save the first number separately
        if let Some(&first) = self.stack.first() {
            self.latex_stack_history = format!("{}", first);
            self.stack_history = format!("{}", first);
            
            // Start from the second number
            for &num in self.stack.iter().skip(1) {
                self.latex_stack_history = format!(r"{{{}}} \cdot {}", self.latex_stack_history, num);
                self.stack_history = format!("({}) * {}", self.stack_history, num);
            }
        }
    
        self.stack.clear();
        self.stack.push(result);
    }

    fn new_number_handling(&mut self, token: &str) {
        if let Ok(num) = token.parse::<f64>() {
            self.stack.push(num);
            if self.indicator{
                self.stack_history = format!("{}", num);
                self.latex_stack_history = format!("{}", num);
            }
        }
    }

    fn welcome_prompt() {
        println!("-----------------------------");
        println!("Welcome to the RPN calculator, please input your equation with 'enter' between!");
        println!("Type 'exit' to quit.");
        println!("-----------------------------");
    }

    fn get_result(&self) -> Option<f64> {
        self.stack.last().cloned()
    }
}

// Main function for executing the RPN calculator
fn main() {
    let mut calc = RPNCalculator::new();
    RPNCalculator::welcome_prompt();
    let mut input = String::new();

    // "Main loop", repeating logic for each input
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting RPN Calculator...");
            println!("Your infix calculation is: {}", calc.stack_history);
            println!("Your LaTeX calculation is: {}", calc.latex_stack_history);

            match calc.get_result() {
                Some(value) => println!("The final result is: {}", value),
                None => println!("No result available."),
            }
            break;
        }

        calc.apply_operation(input);

        match input {
            "+" | "-" | "*" | "/" | "^" | "sqrt" | "log" | "abs" | "++" | "**" | "!" => {
                if let Some(value) = calc.get_result() {
                    println!("The current result is: {}", value);
                }
            }
            _ => {}
        }
    }
}
