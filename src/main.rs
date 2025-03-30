// #![feature(trace_macros)]
// trace_macros!(true);

pub mod math_macros;
pub mod math_edsl;
pub mod svg_edsl;
pub mod svg_macros;

use svg_edsl::SvgCanvas;
use math_edsl::Expr;

fn test(current: Expr, expected: Expr) {
    if !current.is_ast_equals(&expected) {
        panic!("Hier ist ein Fehler");
    }
}

#[macro_export]
macro_rules! add_split {
    ($first:tt $(+ $rest:expr)+) => {{
        let mut v = vec![stringify!($first)];
        $(
            v.push(stringify!($rest));
        )+
        v
    }};
}


fn main() {
    // --------------------math edsl------------------------
    let input = math_expr!((3.0 * (x + 5.0))/(2.0*(sqrt(4.0))));

    println!("Evaluated Expression: {:?}", input.to_string_normal());

    let x_value = 2.0;

    println!("Berechneter Wert f({}) = {}", x_value, input.eval(x_value));

    // LaTeX-Ausgabe mit Fehlerbehandlung
    let latex = input.to_latex();
    println!("LaTeX: {}", latex);

    // Berechnung der Ableitung
    let derivative = input.derivative().simplify().to_string_normal();
    println!("Ableitung f'(x): {}", derivative);

    
    test(math_expr!(4.0), Expr::Num(4.0));

    test(math_expr!(4.0 + 5.0), Expr::Add(Box::new(Expr::Num(4.0)), Box::new(Expr::Num(5.0))));
   
    test(math_expr!(4.0 * ((5.0^2.0) - 3.0)), Expr::Mul(
        Box::new(Expr::Num(4.0)),
        Box::new(Expr::Sub(
            Box::new(Expr::Pow(Box::new(Expr::Num(5.0)), 2.0)),
            Box::new(Expr::Num(3.0))
        ))
    ));
    
    test(math_expr!((4.0 / 2.0) + x), Expr::Add(
        Box::new(Expr::Div(
            Box::new(Expr::Num(4.0)),
            Box::new(Expr::Num(2.0))
        )),
        Box::new(Expr::Var)
    ));

   
    math_expr!((((((x))))));

    
    math_expr!(x);

    math_expr! (5.0 + x);   
    math_expr! ((sqrt(5.0)) + x);

    // --------------------svg edsl------------------------
    svg_elem!(circle(1, 2, 5, "red"));
    svg_elem! (circle(1, 2, 5, "red"));
    svg_elem! (circle(1, 2, 5, "red"));

    let shapes = svg_expr!(  
        circle(80, 70, 30, "purple"),
        circle(50, 40, 40, "red"),
        rect(30, 50, 50, 60, "blue"),
        line(25, 43, 80, 43, "black"),
        text(30, 40, 20, "Hello", "yellow")
    );

    // Definiere eine SVG-Zeichnung im Code (Beispiel)
    let svg = SvgCanvas::new(300, 300, shapes);
    svg.save("output.svg");
}
