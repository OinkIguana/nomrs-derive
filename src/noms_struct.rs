use syn;
use quote;
use heck::MixedCase;

pub fn noms_struct(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let (from_prop_list, to_prop_list) = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => {
            let props: Vec<(syn::Ident, syn::Ident)> = body.iter().filter_map(|field| field.ident.as_ref()).map(|key| (key.clone(), key.as_ref().to_mixed_case().into())).collect();
            let prop_defs = props
                .iter()
                .map(|&(ref key, ref db_key)| quote! {
                    #key: props.remove(stringify!(#db_key))?.transform()
                });
            let from = quote! {
                Some(#name{
                    #(#prop_defs),*
                })
            };
            let map_pairs = props
                .iter()
                .map(|&(ref key, ref db_key)| quote! {
                    map.insert(stringify!(#db_key).to_string(), self.#key);
                });
            let nkeys = props.len();
            let to = quote! {
                let mut map = ::std::collections::HashMap::with_capacity(#nkeys);
                #(#map_pairs)*
                map
            };
            (from, to)
        }
        _ => panic!("Only structs can be NomsStructs!")
    };

    quote! {
        impl<'a> ::nomrs::value::NomsStruct<'a> for #name {
            const NAME: &'static str = stringify!(#name);
            fn from_prop_list(mut props: ::std::collections::HashMap<String, ::nomrs::value::NomsValue<'a>>) -> Option<Self> {
                #from_prop_list
            }
            fn to_prop_list(&self) -> ::std::collections::HashMap<String, ::nomrs::value::NomsValue<'a>> {
                #to_prop_list
            }
        }
    }
}
