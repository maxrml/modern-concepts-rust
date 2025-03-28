#[macro_export]
macro_rules! expr_add {
    ($a:expr, $b:expr) => {
        Expr::Add(Box::new($a), Box::new($b))
    };
}

#[macro_export]
macro_rules! expr_sub {
    ($a:expr, $b:expr) => {
        Expr::Sub(Box::new($a), Box::new($b))
    };
}

#[macro_export]
macro_rules! expr_mul {
    ($a:expr, $b:expr) => {
        Expr::Mul(Box::new($a), Box::new($b))
    };
}

#[macro_export]
macro_rules! expr_div {
    ($a:expr, $b:expr) => {
        Expr::Div(Box::new($a), Box::new($b))
    };
}

#[macro_export]
macro_rules! expr_pow {
    ($a:expr, $n:expr) => {
        Expr::Pow(Box::new($a), $n)
    };
}

#[macro_export]
macro_rules! expr_sqrt {
    ($a:expr) => {
        Expr::Sqrt(Box::new($a))
    };
}

#[macro_export]
macro_rules! expr_num {
    ($val:expr) => {
        Expr::Num($val)
    };
}

#[macro_export]
macro_rules! expr_var {
    () => {
        Expr::Var
    };
}


// In macros.rs

// Expression-Makro für die Mathematischen Operationen
#[macro_export]
macro_rules! expr {
    // Addition
    ($left:expr + $right:expr) => {
        $crate::math_edsl::Expr::Add(Box::new($crate::expr!($left)), Box::new($crate::expr!($right)))
    };
    
    // Subtraktion
    ($left:expr - $right:expr) => {
        $crate::math_edsl::Expr::Sub(Box::new($crate::expr!($left)), Box::new($crate::expr!($right)))
    };

    // Multiplikation
    ($left:expr * $right:expr) => {
        $crate::math_edsl::Expr::Mul(Box::new($crate::expr!($left)), Box::new($crate::expr!($right)))
    };

    // Division
    ($left:expr / $right:expr) => {
        $crate::math_edsl::Expr::Div(Box::new($crate::expr!($left)), Box::new($crate::expr!($right)))
    };

    // Potenzen (exponentiation)
    ($base:expr ^ $exp:expr) => {
        $crate::math_edsl::Expr::Pow(Box::new($crate::expr!($base)), $exp)
    };

    // Quadratwurzel
    (sqrt($inner:expr)) => {
        $crate::math_edsl::Expr::Sqrt(Box::new($crate::expr!($inner)))
    };

    // Variable x
    (x) => {
        $crate::math_edsl::Expr::Var
    };

    // Zahl
    ($num:expr) => {
        $crate::math_edsl::Expr::Num($num)
    };
}
