extern crate heck;
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

mod into_noms;
mod from_noms;
mod noms_struct;

use proc_macro::TokenStream;

#[proc_macro_derive(IntoNoms)]
pub fn into_noms(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = into_noms::into_noms(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(FromNoms)]
pub fn from_noms(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = from_noms::from_noms(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(NomsStruct)]
pub fn noms_struct(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = noms_struct::noms_struct(&ast);
    gen.parse().unwrap()
}
