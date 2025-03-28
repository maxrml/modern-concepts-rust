extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{BinOp, Expr, parse_macro_input};

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Expr);
    transform_expr(&ast).into()
}

fn transform_expr(expr: &Expr) -> TokenStream2 {
    match expr {
        syn::Expr::Binary(binary_expr) => {
            let left = transform_expr(&*binary_expr.left);
            let right = transform_expr(&*binary_expr.right);
            match &binary_expr.op {
                BinOp::Add(_) => quote! { Expr::Add(Box::new(#left), Box::new(#right)) },
                BinOp::Sub(_) => quote! { Expr::Sub(Box::new(#left), Box::new(#right)) },
                BinOp::Mul(_) => quote! { Expr::Mul(Box::new(#left), Box::new(#right)) },
                BinOp::Div(_) => quote! { Expr::Div(Box::new(#left), Box::new(#right)) },
                _ => quote! {},
            }
        }
        _ => quote! {},
    }
}
