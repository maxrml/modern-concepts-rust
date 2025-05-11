pub mod math_edsl;
pub mod math_macros;
pub mod svg_edsl;
pub mod svg_macros;
use svg_edsl::SvgCanvas;

fn main() {
    // --------------------math edsl------------------------
    let input = math_expr!(2.0 + (x - (2.0 * 3.0)));
    // let input = math_expr!((sqrt(x)));
    // let input = math_expr!((3.0 ^ 2.0)/x);
    println!("Evaluated Expression: {:?}", input.to_string_normal());

    let x_value = 2.0;

    println!("Berechneter Wert f({}) = {}", x_value, input.eval(x_value));

    // LaTeX
    let latex = input.to_latex();
    println!("LaTeX: {}", latex);

    // Derivative
    let derivative = input.derivative().simplify().to_string_normal();
    println!("Ableitung f'(x): {}", derivative);

    // --------------------svg edsl------------------------
    let shapes = svg_expr!(
        circle(80, 70, 30, "purple"),               
        circle(50, 40, 40, "red"),                          // circle (x, y, radius, colour)
        rect(30, 50, 50, 60, "blue"),                       // rectangle (x, y, width, height, colour)
        line(25, 43, 80, 43, "black"),                      // line (start x-, y-coordinates, end x-, y-coordinates, colour)
        text(30, 40, 20, "hello", "yellow")                 // text (x , y, font size, text, colour)
    );

    let svg = SvgCanvas::new(300, 300, shapes);
    svg.save("output.svg");
}
