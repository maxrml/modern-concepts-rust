#[macro_export]
macro_rules! expr {
    (($($inner:tt)*)) => {
        expr!($($inner)*)
    };

    // Base case for numbers
    ($num:literal) => {
        crate::math_edsl::Expr::Num($num)
    };

    // Variable handling
    ($x:ident) => {
        crate::math_edsl::Expr::Var
    };

    // Addition
    ($left:tt + $right:tt) => {
        crate::math_edsl::Expr::Add(Box::new(expr!($left)), Box::new(expr!($right)))
    };

    // Subtraction
    ($left:tt - $right:tt) => {
        crate::math_edsl::Expr::Sub(Box::new(expr!($left)), Box::new(expr!($right)))
    };

    // Multiplication
    ($left:tt * $right:tt) => {
        crate::math_edsl::Expr::Mul(Box::new(expr!($left)), Box::new(expr!($right)))
    };

    // Division
    ($left:tt / $right:tt) => {
        crate::math_edsl::Expr::Div(Box::new(expr!($left)), Box::new(expr!($right)))
    };

    // Exponentiation
    ($base:tt ^ $exp:literal) => {
        crate::math_edsl::Expr::Pow(Box::new(expr!($base)), $exp)
    };
}
