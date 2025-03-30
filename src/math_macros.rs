#[macro_export]
macro_rules! math_expr {
    (($($inner:tt)*)) => {
        math_expr!($($inner)*)
    };

    // Base case for numbers
    ($num:literal) => {
        crate::math_edsl::Expr::Num($num)
    };

    // Variable handling
    (x) => {
        crate::math_edsl::Expr::Var
    };

    // Addition
    ($left:tt + $right:tt) => {
        crate::math_edsl::Expr::Add(Box::new(math_expr!($left)), Box::new(math_expr!($right)))
    };

    // Subtraction
    ($left:tt - $right:tt) => {
        crate::math_edsl::Expr::Sub(Box::new(math_expr!($left)), Box::new(math_expr!($right)))
    };

    // Multiplication
    ($left:tt * $right:tt) => {
        crate::math_edsl::Expr::Mul(Box::new(math_expr!($left)), Box::new(math_expr!($right)))
    };

    // Division
    ($left:tt / $right:tt) => {
        crate::math_edsl::Expr::Div(Box::new(math_expr!($left)), Box::new(math_expr!($right)))
    };

    // Exponentiation
    ($base:tt ^ $exp:literal) => {
        crate::math_edsl::Expr::Pow(Box::new(math_expr!($base)), $exp)
    };

    // Sqrt
    (sqrt($val:tt)) => {
        crate::math_edsl::Expr::Sqrt(Box::new(math_expr!($val)))
    }
}
