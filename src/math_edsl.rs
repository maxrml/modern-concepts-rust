#[derive(Clone, Debug)]
//definiion of an enum to represent mathematical expressions
pub enum Expr {
    Num(f64),                    //a number (constant)
    Var,                         //variable x
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, f64),
    Sqrt(Box<Expr>),
}

//evaluates the expression for a given value of x
impl Expr {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Var => x,
            Expr::Add(a, b) => a.eval(x) + b.eval(x),
            Expr::Sub(a, b) => a.eval(x) - b.eval(x),
            Expr::Mul(a, b) => a.eval(x) * b.eval(x),
            Expr::Div(a, b) => a.eval(x) / b.eval(x),
            Expr::Pow(a, n) => a.eval(x).powf(*n),
            Expr::Sqrt(a) => a.eval(x).sqrt(),
        }
    }

    //computes the derivative of the expression
    pub fn derivative(&self) -> Expr {
        match self {
            Expr::Num(_) => Expr::Num(0.0),
            Expr::Var => Expr::Num(1.0),
            Expr::Add(a, b) => Expr::Add(Box::new(a.derivative()), Box::new(b.derivative())),
            Expr::Sub(a, b) => Expr::Sub(Box::new(a.derivative()), Box::new(b.derivative())),
            Expr::Mul(a, b) => Expr::Add(
                Box::new(Expr::Mul(a.clone(), Box::new(b.derivative()))),
                Box::new(Expr::Mul(Box::new(a.derivative()), b.clone())),
            ),
            Expr::Div(a, b) => Expr::Div(
                Box::new(Expr::Sub(
                    Box::new(Expr::Mul(Box::new(a.derivative()), b.clone())),
                    Box::new(Expr::Mul(a.clone(), Box::new(b.derivative()))),
                )),
                Box::new(Expr::Pow(b.clone(), 2.0)),
            ),
            Expr::Pow(a, n) => Expr::Mul(
                Box::new(Expr::Num(*n)),
                Box::new(Expr::Mul(
                    Box::new(a.derivative()),
                    Box::new(Expr::Pow(a.clone(), n - 1.0)),
                )),
            ),
            Expr::Sqrt(a) => Expr::Div(
                Box::new(a.derivative()),
                Box::new(Expr::Mul(
                    Box::new(Expr::Num(2.0)),
                    Box::new(Expr::Sqrt(a.clone())),
                )),
            ),
        }
    }

    //simplifies the expression
    pub fn simplify(&self) -> Expr {
        match self {
            // Constants stay the same
            Expr::Num(n) => Expr::Num(*n),
            Expr::Var => Expr::Var,

            // Simplify addition
            Expr::Add(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();
                match (&a_simp, &b_simp) {
                    (Expr::Num(0.0), _) => b_simp, // 0 + x = x
                    (_, Expr::Num(0.0)) => a_simp, // x + 0 = x
                    _ => Expr::Add(Box::new(a_simp), Box::new(b_simp)),
                }
            }

            // Simplify multiplication
            Expr::Mul(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();
                match (&a_simp, &b_simp) {
                    (Expr::Num(0.0), _) | (_, Expr::Num(0.0)) => Expr::Num(0.0),
                    (Expr::Num(1.0), _) => b_simp,
                    (_, Expr::Num(1.0)) => a_simp,
                    _ => Expr::Mul(Box::new(a_simp), Box::new(b_simp)),
                }
            }

            Expr::Sub(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();
                if let Expr::Num(0.0) = b_simp {
                    return a_simp;
                }
                Expr::Sub(Box::new(a_simp), Box::new(b_simp))
            }

            Expr::Div(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();
                if let Expr::Num(1.0) = b_simp {
                    return a_simp;
                }
                if let Expr::Num(0.0) = a_simp {
                    return Expr::Num(0.0);
                }
                Expr::Div(Box::new(a_simp), Box::new(b_simp))
            }

            Expr::Pow(a, n) => {
                let a_simp = a.simplify();
                if *n == 0.0 {
                    return Expr::Num(1.0);
                }
                if *n == 1.0 {
                    return a_simp;
                }
                Expr::Pow(Box::new(a_simp), *n)
            }

            Expr::Sqrt(a) => {
                let a_simp = a.simplify();
                Expr::Sqrt(Box::new(a_simp))
            }
        }
    }
    //returns a normally formatted string of the expression
    pub fn to_string_normal(&self) -> String {
        match self {
            Expr::Num(n) => format!("{}", n),
            Expr::Var => "x".to_string(),
            Expr::Add(a, b) => format!("({} + {})", a.to_string_normal(), b.to_string_normal()),
            Expr::Sub(a, b) => format!("({} - {})", a.to_string_normal(), b.to_string_normal()),
            Expr::Mul(a, b) => format!("{} * {}", a.to_string_normal(), b.to_string_normal()),
            Expr::Div(a, b) => format!("({} / {})", a.to_string_normal(), b.to_string_normal()),
            Expr::Pow(a, n) => format!("{}^{}", a.to_string_normal(), n),
            Expr::Sqrt(a) => format!("sqrt({})", a.to_string_normal()),
        }
    }

    //returns a LaTeX-formatted string of the expression
    pub fn to_latex(&self) -> String {
        match self {
            Expr::Num(n) => format!("{}", n),
            Expr::Var => "x".to_string(),
            Expr::Add(a, b) => format!("({} + {})", a.to_latex(), b.to_latex()),
            Expr::Sub(a, b) => format!("({} - {})", a.to_latex(), b.to_latex()),
            Expr::Mul(a, b) => format!("{} \\cdot {}", a.to_latex(), b.to_latex()),
            Expr::Div(a, b) => format!("\\frac{{{}}}{{{}}}", a.to_latex(), b.to_latex()),
            Expr::Pow(a, n) => format!("{}^{{{}}}", a.to_latex(), n),
            Expr::Sqrt(a) => format!("\\sqrt{{{}}}", a.to_latex()),
        }
    }
}
