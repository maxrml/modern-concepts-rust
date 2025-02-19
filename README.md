# RPN Calculator Documentation

## Overview
This document provides an overview of the RPN (Reverse Polish Notation) calculator implementation. The calculator allows users to input numbers and operators sequentially and calculates the result accordingly. It maintains an internal stack to perform operations and supports infix notation history and LaTeX-style formatting.

## Function Descriptions

### `main()`
- Initializes an `RPNCalculator` instance.
- Displays the welcome prompt.
- Reads user input in a loop.
- Calls `apply_operation()` for processing.
- Outputs the final result and history when the user inputs `exit`.

### `RPNCalculator::new()`
- Initializes an empty stack, history strings, and an indicator flag.

### `RPNCalculator::apply_operation(&mut self, token: &str)`
- Checks the token and routes to the appropriate function.
- Calls `arithmetical_operation_handling()` for basic math operations (`+`, `-`, `*`, `/`, `^`).
- Calls `log_abs_sqrt_operation_handling()` for unary operations (`sqrt`, `log`, `abs`).
- Calls `factorial_operation_handling()` for factorial computation.
- Calls `full_stack_addition_handling()` for summing the stack (`++`).
- Calls `full_stack_multiplication_handling()` for multiplying the stack (`**`).
- Calls `new_number_handling()` if the token is a number.

### `RPNCalculator::arithmetical_operation_handling(&mut self, token: &str)`
- Pops the last two numbers from the stack.
- Computes the result based on the operator.
- Updates `stack_history` and `latex_stack_history`.
- Pushes the result back onto the stack.

### `RPNCalculator::log_abs_sqrt_operation_handling(&mut self, token: &str)`
- Pops the last number from the stack.
- Computes the result (`sqrt`, `log`, or `abs`).
- Updates history and pushes the result back.

### `RPNCalculator::factorial_operation_handling(&mut self)`
- Pops the last number from the stack.
- Computes the factorial.
- Updates history and pushes the result.

### `RPNCalculator::full_stack_addition_handling(&mut self)`
- Computes the sum of all numbers on the stack.
- Iterates through stack values and updates history.
- Clears the stack and pushes the result.

### `RPNCalculator::full_stack_multiplication_handling(&mut self)`
- Computes the product of all numbers on the stack.
- Iterates through stack values and updates history.
- Clears the stack and pushes the result.

### `RPNCalculator::new_number_handling(&mut self, token: &str)`
- Parses and pushes a new number onto the stack.
- Initializes the history for the first number.

### `RPNCalculator::welcome_prompt()`
- Prints a welcome message.

### `RPNCalculator::get_result(&self) -> Option<f64>`
- Returns the last value on the stack.

---

## Function Interaction Diagram

```mermaid\sequenceDiagram
    participant User
    participant main()
    participant apply_operation()
    participant arithmetical_operation_handling()
    participant log_abs_sqrt_operation_handling()
    participant factorial_operation_handling()
    participant full_stack_addition_handling()
    participant full_stack_multiplication_handling()
    participant new_number_handling()
    participant get_result()

    User ->> main(): Inputs number/operator
    main() ->> apply_operation(): Process input
    
    apply_operation() -->> new_number_handling(): If number
    new_number_handling() ->> apply_operation(): Push to stack
    
    apply_operation() -->> arithmetical_operation_handling(): If `+`, `-`, `*`, `/`, `^`
    arithmetical_operation_handling() ->> get_result(): Compute result
    arithmetical_operation_handling() ->> apply_operation(): Update history & stack
    
    apply_operation() -->> log_abs_sqrt_operation_handling(): If `sqrt`, `log`, `abs`
    log_abs_sqrt_operation_handling() ->> get_result(): Compute result
    log_abs_sqrt_operation_handling() ->> apply_operation(): Update history & stack
    
    apply_operation() -->> factorial_operation_handling(): If `!`
    factorial_operation_handling() ->> get_result(): Compute factorial
    factorial_operation_handling() ->> apply_operation(): Update history & stack
    
    apply_operation() -->> full_stack_addition_handling(): If `++`
    full_stack_addition_handling() ->> get_result(): Compute sum
    full_stack_addition_handling() ->> apply_operation(): Update history & stack
    
    apply_operation() -->> full_stack_multiplication_handling(): If `**`
    full_stack_multiplication_handling() ->> get_result(): Compute product
    full_stack_multiplication_handling() ->> apply_operation(): Update history & stack
    
    User ->> main(): Inputs `exit`
    main() ->> get_result(): Fetch final result
    main() ->> User: Display infix, LaTeX, and result
```

