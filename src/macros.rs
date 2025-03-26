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
