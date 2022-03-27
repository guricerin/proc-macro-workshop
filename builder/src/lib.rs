use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput,Ident};
use quote::quote;
use proc_macro2::Span;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let ty_name = input.ident;
    let builder_name = Ident::new(&format!("{}Builder", ty_name),Span::call_site());

    // Build the output, possibly using quasi-quotation
    let expanded = quote!{
        pub struct #builder_name {

        }

        impl #ty_name {
            pub fn builder () -> #builder_name {
                #builder_name {}
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
