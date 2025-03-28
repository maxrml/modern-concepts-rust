pub mod macros;
pub mod math_edsl;
pub mod svg_edsl;

use my_macro_lib::expr;
use math_edsl::Expr;
use svg_edsl::{Shape, SvgCanvas};

fn main() {
    // Definiere die mathematische Funktion und den x-Wert im Code
    let input = expr!(x^2 + 3*x - 5); // EDSL-Syntax!
    let x_value = 2.0;

    println!("Berechneter Wert f({}) = {}", x_value, expr.eval(x_value));


    // LaTeX-Ausgabe mit Fehlerbehandlung
    let latex = expr.to_latex();
    println!("LaTeX: {}", latex);

    // Berechnung der Ableitung
    let derivative = expr.derivative().simplify().to_string_normal();
    println!("Ableitung f'(x): {}", derivative);

    // Definiere eine SVG-Zeichnung im Code (Beispiel)
    let mut svg = SvgCanvas::new(500, 500);
    svg.add_function_graph(&expr, "blue");
    svg.add_shape(Shape::Circle { cx: 250, cy: 250, r: 50, fill: "red".to_string() });

    println!("SVG Ausgabe: {}", svg.to_svg());
}
