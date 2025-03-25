pub mod math_edsl;
pub mod svg_edsl;

use math_edsl::Expr;
use std::io;
use svg_edsl::{Shape, SvgCanvas};


fn main() {
    // --- Mathematische eDSL ---
    //define 3 functions
    let f1 = Expr::Sqrt(Box::new(Expr::Add(
        Box::new(Expr::Pow(Box::new(Expr::Var), 2.0)),
        Box::new(Expr::Num(1.0)),                                      
    )));

    let f2 = Expr::Div(
        Box::new(Expr::Sub(
            Box::new(Expr::Pow(Box::new(Expr::Var), 3.0)),
            Box::new(Expr::Num(5.0)),
        )),
        Box::new(Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(2.0)))),
    );

    let f3 = Expr::Mul(
        Box::new(Expr::Sqrt(Box::new(Expr::Var))),
        Box::new(Expr::Add(Box::new(Expr::Var), Box::new(Expr::Num(4.0)))),
    );

    //display functions for user
    let functions = vec![("sqrt(x^2 + 1)", f1),
                                            ("(x^3 - 5) / (x + 2)", f2),
                                            ("sqrt(x) * (x + 4)", f3)];
                                            
    //Let the user choose a function
    println!("Choose a function to evaluate:");
    for (i, (desc, _)) in functions.iter().enumerate() {
        println!("{}. {}", i + 1, desc);
    }

    //read user input
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error reading input");
    let choice: usize = choice.trim().parse::<usize>().expect("Invalid choice!") - 1;

    //throw error if users number is too high
    if choice >= functions.len() {
        println!("Invalid choice, pick an excisting function");
        return;
    }

    let(_, selected_f) = &functions[choice];

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
    println!("Derivative f'(x): {}", selected_f.derivative().simplify().to_string_normal());
    
    //plot the graphs f(x) and its derivative on the svg canvas
    // println!("Would you like to plot f(x) and its derivative on the SVG canvas? (yes/no)");
    // let mut plot_choice = String::new();
    // io::stdin().read_line(&mut plot_choice).expect("Error reading input");
    // plot_choice = plot_choice.trim().to_lowercase();



    // --- SVG eDSL ---
    //create a canvas on which the user will draw on
    let mut canvas = SvgCanvas::new(300, 300);

    //plot the points of f(x) and its derivative
    // if plot_choice == "yes"{
    //     let function_points = SvgCanvas::generate_function_points(selected_f, "blue");
    //     let derivative_points = SvgCanvas::generate_function_points(&selected_f.derivative(), "red");

    //     for point in function_points {
    //         canvas.add_shape(point);
    //     }

    //     for point in derivative_points {
    //         canvas.add_shape(point);
    //     }

    //     let x_axis = Shape::Line {
    //         x1: 0,
    //         y1: 150,
    //         x2: 300,
    //         y2: 150,
    //         stroke: "black".to_string(),
    //     };
    //     canvas.add_shape(x_axis);
    
        
    //     let y_axis = Shape::Line {
    //         x1: 150,
    //         y1: 0,
    //         x2: 150,
    //         y2: 300,
    //         stroke: "black".to_string(),
    //     };
    //     canvas.add_shape(y_axis);
    // }

    //loop through shape, colour, coordinates and size, to let user "draw" multiptle shapes
    loop {
        //user input shape
        println!("Which form would you like to draw (circle, rect, line, text, exit to end the program):");
        let mut shape_type = String::new();
        io::stdin().read_line(&mut shape_type).expect("Error reading input");
        let shape_type = shape_type.trim().to_lowercase();

        if shape_type == "exit" {
            break;
        }

        //user input color
        println!("What colour should it be?");
        let mut color = String::new();
        io::stdin().read_line(&mut color).expect("Error reading input");
        let color = color.trim().to_string();

        match shape_type.as_str() {
            //if user input was circle, get user input for coordinates and radius
            "circle" => {
                println!("Enter center x-coordinate, center y-coordinate and radius (z.B. 100 100 50):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Error reading input");
                let values: Vec<i32> = input.trim().split_whitespace()
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
                io::stdin().read_line(&mut input).expect("Error reading input");
                let values: Vec<i32> = input.trim().split_whitespace()
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
                println!("Enter start x- and y-coordinate, end x- and y-coordinate (e.g. 50 200 350 200):");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Error reading input");
                let values: Vec<i32> = input.trim().split_whitespace()
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
                io::stdin().read_line(&mut input).expect("Error reading input");
                let values: Vec<i32> = input.trim().split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                println!("Enter the text content:");
                let mut text_content = String::new();
                io::stdin().read_line(&mut text_content).expect("Error reading input");
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
