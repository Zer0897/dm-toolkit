extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Unit)]
pub fn unit_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_unit_macro(&ast)
}

// Inspired by units https://github.com/Peternator7/strum/blob/master/strum_macros/src/macros/enum_iter.rs
fn impl_unit_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("Unit only works on Enums"),
    };

    let mut units = Vec::new();
    for variant in variants.iter() {
        use syn::Fields::*;
        let ident = &variant.ident;
        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(ref fields) => {
                let defaults = ::std::iter::repeat(quote!(::std::default::Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(ref fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: ::std::default::Default::default()),*} }
            }
        };

        units.push(quote! { #name::#ident #params });
    }
    let variant_count = units.len();
    let gen = quote! {
        impl CountUnit for #name {
            type Item = #name;
            fn units() -> std::slice::Iter<'static, #name> {
                static UNITS: [#name; #variant_count]  = [#(#units),*];
                UNITS.into_iter()
            }
        }
    };
    gen.into()
}
