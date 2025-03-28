pub mod macros;
pub mod math_edsl;
pub mod svg_edsl;

use my_macro_lib::calculate_expr;
use svg_edsl::{Shape, SvgCanvas};

fn main() {
    // Definiere die mathematische Funktion und den x-Wert im Code
    let input = expr!(3 + 4 * (2 - 1));

    println!("Evaluated Expression: {:?}", input);

    let x_value = 2.0;

    println!("Berechneter Wert f({}) = {}", x_value, input.eval(x_value));

    // LaTeX-Ausgabe mit Fehlerbehandlung
    let latex = input.to_latex();
    println!("LaTeX: {}", latex);

    // Berechnung der Ableitung
    let derivative = input.derivative().simplify().to_string_normal();
    println!("Ableitung f'(x): {}", derivative);

    // Definiere eine SVG-Zeichnung im Code (Beispiel)
    let mut svg = SvgCanvas::new(500, 500);
    svg.add_function_graph(&input, "blue");
    svg.add_shape(Shape::Circle {
        cx: 250,
        cy: 250,
        r: 50,
        fill: "red".to_string(),
    });

    println!("SVG Ausgabe: {}", svg.to_svg());
}
