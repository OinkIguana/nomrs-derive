use syn;
use quote;

pub fn from_noms(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl<'a> ::nomrs::value::FromNoms<'a> for #name {
            fn from_noms(chunk: &::nomrs::Chunk) -> Self {
                ::nomrs::value::NomsValue::from_noms(chunk).transform_struct()
            }
        }
    }
}
