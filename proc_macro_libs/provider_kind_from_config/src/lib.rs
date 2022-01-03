use std::collections::HashMap;
use std::fs;

use convert_case::Case;
use convert_case::Casing;

use quote::quote;
use syn;

use proc_macro::TokenStream;

use crate::syn::Data;
use crate::syn::Ident;

#[proc_macro_derive(ProviderKindFromConfigTraitDerive)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap(); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let mut function_hashmap = HashMap::new();
    match fs::read_to_string("./src/traits/provider_kind_from_config_trait.rs") {
        Err(e) => panic!("file:  error: {}", e),
        Ok(file) => {
            //todo parse into TokenStream
            let mut string_to_parse = file;
            let start_symbol = "fn ";
            let end_symbol = ";";
            let end_function_name_symbol = "(&self) -> ";
            while string_to_parse.contains(start_symbol) && string_to_parse.contains(end_symbol) {
                if let Some(start_index) = string_to_parse.find(start_symbol) {
                    if let Some(end_index) = string_to_parse.find(end_symbol) {
                        if end_index > start_index {
                            let function_string = string_to_parse
                                [start_index + start_symbol.len()..end_index]
                                .to_string();
                            match function_string.find(end_function_name_symbol) {
                                Some(end_function_name_index) => {
                                    if end_function_name_index + end_function_name_symbol.len()
                                        >= end_index
                                    {
                                        panic!("end_function_name_index + end_function_name_symbol.len() >= end_index");
                                    }
                                    let function_name =
                                        function_string[..end_function_name_index].to_string();
                                    let function_return_type = function_string
                                        [end_function_name_index
                                            + end_function_name_symbol.len()..]
                                        .to_string();
                                    function_hashmap.insert(function_name, function_return_type);
                                }
                                None => panic!("no end_function_name_symbol"),
                            }
                            string_to_parse =
                                string_to_parse[end_index + end_symbol.len()..].to_string();
                        } else {
                            panic!("end_index > start_index");
                        }
                    }
                }
            }
        }
    }
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = data {
        variants
    } else {
        unimplemented!();
    };
    let mut function_quote_vec = Vec::new();
    for (name, return_type) in function_hashmap {
        let function_name_ident = syn::Ident::new(
            &name.to_string().to_case(Case::Snake).to_lowercase(),
            ident.span(),
        );
        let function_return_type_ident = syn::Ident::new(&return_type, ident.span());
        let variants_for_quote = variants.iter().map(|f| {
            let variant_name = &f.ident;
            let config_field_name = syn::Ident::new(
                &format!(
                    "{}_{}",
                    name.to_string().to_case(Case::Snake).to_lowercase(),
                    variant_name.to_string().to_case(Case::Snake).to_lowercase()
                ),
                variant_name.span(),
            );
            if function_return_type_ident == "String" {
                //coz "&'static str is not a valid identifier"
                quote! {
                    #ident::#variant_name => CONFIG.#config_field_name.clone()
                }
            } else {
                quote! {
                    #ident::#variant_name => CONFIG.#config_field_name
                }
            }
        });
        let function_quote = quote! {
            fn #function_name_ident(&self) -> #function_return_type_ident {
                match self {
                   #(#variants_for_quote,)*
                }
            }
        };
        function_quote_vec.push(function_quote);
    }
    let generated = quote! {
        impl ProviderKindFromConfigTrait for #ident {
            #(#function_quote_vec)*
        }
    };
    generated.into()
}
