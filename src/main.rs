pub mod macros;
pub mod math_edsl;
pub mod svg_edsl;

use math_edsl::Expr;
use std::io;
use svg_edsl::{Shape, SvgCanvas};

fn main() {
    // --- Mathematische eDSL ---
    //define 3 functions
    let f1 = expr_sqrt!(expr_add!(expr_pow!(expr_var!(), 2.0), expr_num!(1.0)));

    let f2 = expr_div!(
        expr_sub!(expr_pow!(expr_var!(), 3.0), expr_num!(5.0)),
        expr_add!(expr_var!(), expr_num!(2.0))
    );

    let f3 = expr_mul!(
        expr_sqrt!(expr_var!()),
        expr_add!(expr_var!(), expr_num!(4.0))
    );

    //display functions for user
    let functions = vec![
        ("sqrt(x^2 + 1)", f1),
        ("(x^3 - 5) / (x + 2)", f2),
        ("sqrt(x) * (x + 4)", f3),
    ];

    //Let the user choose a function
    println!("Choose a function to evaluate:");
    for (i, (desc, _)) in functions.iter().enumerate() {
        println!("{}. {}", i + 1, desc);
    }

    //read user input
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error reading input");
    let choice: usize = choice.trim().parse::<usize>().expect("Invalid choice!") - 1;

    //throw error if users number is too high
    if choice >= functions.len() {
        println!("Invalid choice, pick an excisting function");
        return;
    }

    let (_, selected_f) = &functions[choice];

    //Display the selected function in LaTeX format
    println!("Selected function: {}", selected_f);

    //get x value from the user
    println!("Enter a number for x:");

    //read user input & throw error if input is invalid
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let x_value: f64 = input.trim().parse().expect("Invalid number!");

    //display results
    println!("f({}) = {}", x_value, selected_f.eval(x_value));
    println!("LaTeX: {}", selected_f.to_latex());
    println!(
        "Derivative f'(x): {}",
        selected_f.derivative().simplify().to_string_normal()
    );

    // --- SVG eDSL ---
    //create a canvas on which the user will draw on
    let mut canvas = SvgCanvas::new(300, 300);

    //loop through shape, colour, coordinates and size, to let user "draw" multiptle shapes
    loop {
        //user input shape
        println!(
            "Which form would you like to draw (circle, rect, line, text, exit to end the program):"
        );
        let mut shape_type = String::new();
        io::stdin()
            .read_line(&mut shape_type)
            .expect("Error reading input");
        let shape_type = shape_type.trim().to_lowercase();

        if shape_type == "exit" {
            break;
        }

        //user input color
        println!("What colour should it be?");
        let mut color = String::new();
        io::stdin()
            .read_line(&mut color)
            .expect("Error reading input");
        let color = color.trim().to_string();

        match shape_type.as_str() {
            //if user input was circle, get user input for coordinates and radius
            "circle" => {
                println!(
                    "Enter center x-coordinate, center y-coordinate and radius (z.B. 100 100 50):"
                );
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");
                let values: Vec<i32> = input
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if values.len() == 3 {
                    canvas.add_shape(Shape::Circle {
                        cx: values[0],
                        cy: values[1],
                        r: values[2],
                        fill: color.clone(),
                    });
                }
            }
            //if user input was rect, get user input for coordinates, width and height
            "rect" => {
                println!("Enter x-coordinate, y-coordinate, width, height (z.B. 50 50 30 30):");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");
                let values: Vec<i32> = input
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if values.len() == 4 {
                    canvas.add_shape(Shape::Rect {
                        x: values[0],
                        y: values[1],
                        width: values[2],
                        height: values[3],
                        fill: color.clone(),
                    });
                }
            }
            //if user input was line, get user input for start- and end-coordinates
            "line" => {
                println!(
                    "Enter start x- and y-coordinate, end x- and y-coordinate (e.g. 50 200 350 200):"
                );
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");
                let values: Vec<i32> = input
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if values.len() == 4 {
                    canvas.add_shape(Shape::Line {
                        x1: values[0],
                        y1: values[1],
                        x2: values[2],
                        y2: values[3],
                        stroke: color.clone(),
                    });
                }
            }
            //if user input was text, get user input for coordinates, font size and the text content
            "text" => {
                println!("Enter x-coordinate, y-coordinate and font size for the text:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");
                let values: Vec<i32> = input
                    .trim()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                println!("Enter the text content:");
                let mut text_content = String::new();
                io::stdin()
                    .read_line(&mut text_content)
                    .expect("Error reading input");
                let text_content = text_content.trim().to_string();

                canvas.add_shape(Shape::Text {
                    x: values[0],
                    y: values[1],
                    size: values[2],
                    fill: color.clone(),
                    text: text_content,
                });
            }
            _ => println!("Invalid form!"),
        }
    }

    //saves the canvas after all shapes have been added as output.svg
    canvas.save("output.svg");

    println!("SVG was safed as 'output.svg'.");
}
