// Library for handling user inputs
use std::io;

// Class declaration
#[derive(Clone)]
struct RPNCalculator {
    stack: Vec<f64>,
    history_stack: Vec<String>,
}

impl RPNCalculator {
    // Constructor, initializing the vectors
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            history_stack: Vec::new(),
        }
    }

    // Applying operation to the stack
    fn apply_operation(&mut self, token: &str) {
        self.history_stack.push(token.to_owned());
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
        self.stack.push(result);
    }

    fn log_abs_sqrt_operation_handling(&mut self, token: &str) {
        let a: f64 = self.stack.pop().unwrap();
        let result = match token {
            "sqrt" => a.sqrt(),
            "log" => a.log10(),
            "abs" => a.abs(),
            _ => unreachable!(),
        };

        self.stack.push(result);
    }

    fn factorial_operation_handling(&mut self) {
        let a = self.stack.pop().unwrap();
        let result = (1..=a as u64).product::<u64>() as f64;

        self.stack.push(result);
    }

    fn full_stack_addition_handling(&mut self) {
        let result: f64 = self.stack.iter().sum();

        self.stack.clear();
        self.stack.push(result);
    }

    fn full_stack_multiplication_handling(&mut self) {
        let result: f64 = self.stack.iter().product();

        self.stack.clear();
        self.stack.push(result);
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

    fn reconstruct_expression_infix(&mut self) -> String {
        if let Some(token) = self.history_stack.pop() {
            match token.as_str() {
                "+" | "-" | "*" | "/" | "^" => {
                    let right = self.reconstruct_expression_infix();
                    let left = self.reconstruct_expression_infix();
                    format!("({} {} {})", left, token, right)
                }
                "++" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_infix());
                    }
                    format!("({})", terms.join(" + "))
                }
                "**" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_infix());
                    }
                    format!("({})", terms.join(" * "))
                }
                "!" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("({}!)", operand)
                }
                "abs" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("abs({})", operand)
                }
                "sqrt" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("sqrt({})", operand)
                }
                "log" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("log10({})", operand)
                }
                _ => token, // Falls es eine Zahl ist
            }
        } else {
            String::new() // Falls der Stack leer ist
        }
    }

    fn reconstruct_expression_latex(&mut self) -> String {
        if let Some(token) = self.history_stack.pop() {
            match token.as_str() {
                "+" | "-" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    // Gesamter Ausdruck in einer Klammerngruppe
                    format!("{{{} {} {}}}", left, token, right)
                }
                "*" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    // Gruppierung analog zu +, aber mit \cdot
                    format!("{{{} \\cdot {}}}", left, right)
                }
                "/" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    // \frac benötigt eigene Gruppierung der Zähler und Nenner, aber man kann
                    // trotzdem die Gesamtdarstellung in eine äußere Klammer einschließen, wenn gewünscht:
                    format!("{{\\frac{{{}}}{{{}}}}}", left, right)
                }
                "^" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    // Hier wird der Exponent ebenfalls in eigenen Klammern gesetzt
                    format!("{{{}^{{{}}}}}", left, right)
                }
                "++" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_latex());
                    }
                    // Gesamte Summenrechnung gruppieren
                    format!("{{{}}}", terms.join(" + "))
                }
                "**" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_latex());
                    }
                    // Gesamte Multiplikation gruppieren
                    format!("{{{}}}", terms.join(" \\cdot "))
                }
                "!" => {
                    let operand = self.reconstruct_expression_latex();
                    // Fakultät: Operand in Klammern, dann !
                    format!("{{{}}}!", operand)
                }
                "abs" => {
                    let operand = self.reconstruct_expression_latex();
                    // Absolutwert mit LaTeX-Syntax
                    format!(r"\left| {{{}}} \right|", operand)
                }
                "sqrt" => {
                    let operand = self.reconstruct_expression_latex();
                    // Quadratwurzel: Ausdruck in Klammern
                    format!(r"\sqrt{{{}}}", operand)
                }
                "log" => {
                    let operand = self.reconstruct_expression_latex();
                    // Logarithmus zur Basis 10
                    format!(r"\log_{{10}} {{{}}}", operand)
                }
                _ => token, // Falls es eine Zahl ist
            }
        } else {
            String::new() // Falls der Stack leer ist
        }
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
            println!("Your infix calculation is: {}", calc.clone().reconstruct_expression_infix());
            println!("Your LaTeX calculation is: {}", calc.clone().reconstruct_expression_latex());

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
