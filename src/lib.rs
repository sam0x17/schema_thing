use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Error, ItemStruct};

#[proc_macro_derive(Schema)]
pub fn derive_schema(tokens: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(tokens as ItemStruct);
    let mut lines: Vec<TokenStream2> = Vec::new();
    for field in item_struct.fields {
        let Some(field_ident) = field.ident else {
            return Error::new(field.span(), "tuple structs not supported").into_compile_error().into()
        };
        let key = field_ident.to_string();
        let value = field.ty.to_token_stream().to_string().trim().to_string();
        lines.push(quote!(map.insert(#key.to_string(), #value.to_string());));
    }
    let item_ident = item_struct.ident;
    quote! {
        impl Schema for #item_ident {
            fn get_schema() -> HashMap<String, String> {
                let mut map: HashMap<String, String> = HashMap::new();
                #(#lines)
                *
                map
            }
        }
    }
    .into()
}
