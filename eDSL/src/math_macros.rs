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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math_edsl::Expr;

    #[test]
    fn test_var() {
        let expr = math_expr!(x);
        assert_eq!(expr.to_string_normal(), "x");
    }

    #[test]
    fn test_number() {
        let expr = math_expr!(5.0);
        assert_eq!(expr.to_string_normal(), "5");
    }

    #[test]
    fn test_addition_macro() {
        let expr = math_expr!(2.0 + x);
        assert_eq!(expr.to_string_normal(), "(2 + x)");
    }

    #[test]
    fn test_subtraction_macro() {
        let expr = math_expr!(2.0 - x);
        assert_eq!(expr.to_string_normal(), "(2 - x)");
    }

    #[test]
    fn test_multiplication_macro() {
        let expr = math_expr!(x* (2.0 * 3.0));
        assert_eq!(expr.to_string_normal(), "x * 2 * 3");
    }

    #[test]
    fn test_division_macro() {
        let expr = math_expr!(2.0 / x);
        assert_eq!(expr.to_string_normal(), "(2 / x)");
    }

    #[test]
    fn test_sqrt_macro() {
        let expr = math_expr!((sqrt(x)));
        assert_eq!(expr.to_string_normal(), "sqrt(x)");
    }

    #[test]
    fn test_nested_addition_and_subtraction() {
        let expr = math_expr!((x + 2.0) - (4.0 + 3.0));
        assert_eq!(expr.to_string_normal(), "((x + 2) - (4 + 3))");
    }

    #[test]
    fn test_empty_expression() {
        let expr = math_expr!(x + x);
        assert_eq!(expr.to_string_normal(), "(x + x)");
    }
}
