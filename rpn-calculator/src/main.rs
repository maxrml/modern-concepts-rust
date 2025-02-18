// Library for handling user inputs
use std::io;

// Class declaration
struct RPNCalculator {
    stack: Vec<f64>,
    latex_stack_history: Vec<String>,
    stack_history: Vec<String>
}

impl RPNCalculator {
    // Constructor, initializing the vectors
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            latex_stack_history: Vec::new(),
            stack_history: Vec::new(),
        }
    }

    // Applying operation the the stack
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

                let infix = format!("({} {} {})", a, token, b);
                let latex = format!("({} {} {})", a, token, b);

                self.stack.push(result);
                self.stack_history.push(infix);
                self.latex_stack_history.push(latex);
    }

    fn log_abs_sqrt_operation_handling(&mut self, token: &str) {
        let a = self.stack.pop().unwrap();
                let result = match token {
                    "sqrt" => a.sqrt(),
                    "log" => a.log10(),
                    "abs" => a.abs(),
                    _ => unreachable!(),
                };

                let infix = format!("{}({})", token, a);
                let latex = match token {
                    "sqrt" => format!(r"\sqrt{{{}}}", a),
                    "log" => format!(r"\log_{{10}}({})", a),
                    "abs" => format!(r"\left| {} \right|", a),
                    _ => unreachable!(),
                };

                self.stack.push(result);
                self.stack_history.push(infix);
                self.latex_stack_history.push(latex);
    }

    fn factorial_operation_handling(&mut self) {
        let a = self.stack.pop().unwrap();
                let result = (1..=a as u64).product::<u64>() as f64;

                let infix = format!("({}!)", a);
                let latex = format!(r"{}!", a);

                self.stack.push(result);
                self.stack_history.push(infix);
                self.latex_stack_history.push(latex);
    }

    fn full_stack_addition_handling(&mut self) {
        let result: f64 = self.stack.iter().sum();
                self.stack.clear();
                self.stack.push(result);
                self.stack_history.push(format!("sum({:?})", self.stack));
                self.latex_stack_history.push(format!(r"\sum {}", result));
    }

    fn full_stack_multiplication_handling(&mut self) {
        let result: f64 = self.stack.iter().product();
                self.stack.clear();
                self.stack.push(result);
                self.stack_history.push(format!("prod({:?})", self.stack));
                self.latex_stack_history.push(format!(r"\prod {}", result));
    }

    fn new_number_handling(&mut self, token: &str) {
        if let Ok(num) = token.parse::<f64>() {
            self.stack.push(num);
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
            println!("Your infinix calculation is: {:?}", concatenate_stack_history(&calc.stack_history));
            println!("Your latex calculation is: {:?}", concatenate_stack_history(&calc.latex_stack_history));

            if let Some(result) = calc.get_result() {
                println!("The final result is: {}", result);
            } else {
                println!("No result available.");
            }

            break;
        }
        calc.apply_operation(input);
        match input {
            "+" | "-" | "*" | "/" | "^" | "sqrt" | "log" | "abs" | "++" | "**" | "!" => {
                println!("Current result: {:?}", calc.get_result())
            }
            _ => {

            }
        }
    }

}

fn concatenate_stack_history(history: &Vec<String>) -> String {
    let mut result = String::new();

    for expression in history {
        result += expression;
    }

    result
}