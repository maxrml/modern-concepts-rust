extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Expr, BinOp};

#[proc_macro]
pub fn calculate_expr(input: TokenStream) -> TokenStream {
    let input_expr = parse_macro_input!(input as Expr);
    transform(input_expr).into()
    }

    fn transform(expr: Expr) -> TokenStream2 {
        match expr {
        Expr::Binary(binary_expr) => handle_binary_expr(binary_expr),
        // Handle additional expression types as needed
        _ => quote! {},
        }
        }

        fn handle_binary_expr(binary_expr: syn::ExprBinary) -> TokenStream2 {
            let left = transform(*binary_expr.left);
            let right = transform(*binary_expr.right);
            match binary_expr.op {
                BinOp::Add(_) => quote! { Expr::Add(Box::new(#left), Box::new(#right)) },
                BinOp::Sub(_) => quote! { Expr::Sub(Box::new(#left), Box::new(#right)) },
                BinOp::Mul(_) => quote! { Expr::Mul(Box::new(#left), Box::new(#right)) },
                BinOp::Div(_) => quote! { Expr::Div(Box::new(#left), Box::new(#right)) },
                _ => quote! {},
            }
            }
