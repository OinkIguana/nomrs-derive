use syn;
use quote;

pub fn into_noms(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ::nomrs::value::IntoNoms for #name {
            fn into_noms(&self) -> Vec<u8> {
                // TODO: implement!!!
                unimplemented!()
            }
        }
    }
}
