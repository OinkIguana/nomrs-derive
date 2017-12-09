#![recursion_limit = "128"]

extern crate heck;
extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

mod into_noms;
mod from_noms;
mod noms_struct;

use proc_macro::TokenStream;

#[proc_macro_derive(Noms)]
pub fn noms(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let into = into_noms::into_noms(&ast);
    let from = from_noms::from_noms(&ast);
    let stru = noms_struct::noms_struct(&ast);

    let gen = quote! {
        #into
        #from
        #stru
    };
    gen.parse().unwrap()
}
