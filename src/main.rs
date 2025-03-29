// #![feature(trace_macros)]
// trace_macros!(true);

pub mod macros;
pub mod math_edsl;
pub mod svg_edsl;

use my_macro_lib::calculate_expr;
use svg_edsl::{Shape, SvgCanvas};
use math_edsl::Expr;


fn test(current: Expr, expected: Expr) {
    if !current.is_ast_equals(&expected) {
        panic!("Hier ist ein Fehler");
    }
}

fn main() {
    // Definiere die mathematische Funktion und den x-Wert im Code

    test(expr!(4.0), Expr::Num(4.0));
    test(expr!(4.0 + 5.0), Expr::Add(Box::new(Expr::Num(4.0)), Box::new(Expr::Num(5.0))));
    test(expr!(4.0 * ((5.0^2.0) - 3.0)), Expr::Mul(
        Box::new(Expr::Num(4.0)),
        Box::new(Expr::Sub(
            Box::new(Expr::Pow(Box::new(Expr::Num(5.0)), 2.0)),
            Box::new(Expr::Num(3.0))
        ))
    ));
    
    test(expr!((4.0 / 2.0) + x), Expr::Add(
        Box::new(Expr::Div(
            Box::new(Expr::Num(4.0)),
            Box::new(Expr::Num(2.0))
        )),
        Box::new(Expr::Var)
    ));

    let input = expr!(3.0 * (x + 5.0));

    expr!((((((x))))));
    expr!(asdjfa);

    // Add(3.0, (1.0) + (1.0))

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
