use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[cfg(feature = "dsl")]
#[proc_macro]
pub fn value(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let expanded = match &expr {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Int(_) => {
                quote! {
                    Box::new(Expr::Value(Value::Number(#expr)))
                }
            }
            // TODO string
            syn::Lit::Bool(_) => {
                quote! {
                    Box::new(Expr::Value(Value::Bool(#expr)))
                }
            }
            _ => {
                quote! {
                    compile_error!("Unsupported literal type {}", )
                }
            }
        },
        Expr::Macro(macro_call) => {
            let macro_ident = &macro_call.mac.path;
            let macro_tokens = &macro_call.mac.tokens;

            quote! {
                #macro_ident!#macro_tokens
            }
        }
        Expr::Unary(unary) => match &unary.op {
            syn::UnOp::Neg(_) => {
                quote! {
                    Box::new(Expr::Value(Value::Number(#expr)))
                }
            }
            _ => {
                quote! {
                    compile_error!("Unsupported unary expression");
                }
            }
        },
        _ => {
            quote! {
                compile_error!("Unsupported expression type")
            }
        }
    };

    TokenStream::from(expanded)
}
