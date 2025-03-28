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



#[macro_export]
macro_rules! expr {
// Base case for numbers
($num:literal) => {
    Expr::Num($num)
    };
    
    // Variable handling
    (x) => {
    Expr::Var
    };
    
    // Addition
    (($left:tt) + ($right:tt)) => {
    Expr::Add(Box::new(expr!($left)), Box::new(expr!($right)))
    };
    
    // Subtraction
    (($left:tt) - ($right:tt)) => {
    Expr::Sub(Box::new(expr!($left)), Box::new(expr!($right)))
    };
    
    // Multiplication
    (($left:tt) * ($right:tt)) => {
    Expr::Mul(Box::new(expr!($left)), Box::new(expr!($right)))
    };
    
    // Division
    (($left:tt) / ($right:tt)) => {
    Expr::Div(Box::new(expr!($left)), Box::new(expr!($right)))
    };
    
    // Exponentiation
    (($base:tt) ^ ($exp:tt)) => {
    Expr::Pow(Box::new(expr!($base)), Box::new(expr!($exp)))
    };
    }
    