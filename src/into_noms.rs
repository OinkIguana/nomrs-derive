use syn;
use quote;

pub fn into_noms(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ::nomrs::value::IntoNoms for #name {
            fn into_noms(&self) -> Vec<u8> {
                let name = stringify!(#name);
                let props = ::nomrs::value::NomsStruct::to_prop_list(self);
                let mut data = vec![9];
                data.extend(::nomrs::util::varint::encode_u64(name.len() as u64));
                data.extend_from_slice(name.as_bytes());
                data.extend(::nomrs::util::varint::encode_u64(props.len() as u64));
                for (key, value) in &props {
                    data.extend(::nomrs::util::varint::encode_u64(key.len() as u64));
                    data.extend_from_slice(key.as_bytes());
                    data.extend(value.into_noms());
                }
                data
            }
        }
    }
}
