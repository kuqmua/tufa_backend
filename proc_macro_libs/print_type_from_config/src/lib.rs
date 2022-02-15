use std::fs;

use quote::quote;

use syn;
use syn::ReturnType;
use syn::TraitItem;

use proc_macro::TokenStream;

use syn::Data;
use syn::Ident;

//copy of provider_kind_from_config
#[proc_macro_derive(PrintTypeFromConfigTraitDerive)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("cannot parse input into syn::DeriveInput"); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let trait_name: Ident;
    let function_vec_idents: Vec<(Ident, ReturnType)>;
    //todo: how to match it without fs::read_to_string("./src/traits/print_type_from_config_trait.rs") ? import CONFIG type somehow?
    match fs::read_to_string("./src/traits/print_type_from_config_trait.rs") {
        Err(e) => panic!("file:  error: {}", e),
        Ok(file) => {
            let token_stream: proc_macro::TokenStream = file
                .parse()
                .expect("cannot parse file into proc_macro::TokenStream");
            let trait_ast: syn::ItemTrait = syn::parse(token_stream)
                .expect("cannot parse token_stream from file into syn::ItemTrait");
            trait_name = trait_ast.ident;
            function_vec_idents = trait_ast
                .items
                .iter()
                .filter_map(|trait_item| match trait_item {
                    TraitItem::Method(trait_item_method) => Some((
                        trait_item_method.sig.ident.clone(),
                        trait_item_method.sig.output.clone(),
                    )),
                    _ => None,
                })
                .collect();
        }
    }
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = data {
        variants
    } else {
        panic!("not a valid data type for this proc macro");
    };
    let mut function_quote_vec_ident = Vec::with_capacity(function_vec_idents.len());
    for (function_name_ident, output) in function_vec_idents {
        let variants_for_quote = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            quote! {
                #ident::#variant_name => CONFIG.#function_name_ident
            }
        });
        function_quote_vec_ident.push(quote! {
            fn #function_name_ident(&self) #output {
                match self {
                   #(#variants_for_quote,)*
                }
            }
        });
    }
    let generated = quote! {
        impl #trait_name for #ident {
            #(#function_quote_vec_ident)*
        }
    };
    generated.into()
}
