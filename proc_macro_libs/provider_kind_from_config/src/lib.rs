use convert_case::Case;
use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn;
use syn::Data;
use syn::Ident;
use syn::ReturnType;
use syn::TraitItem;

//copy of print_type_from_config
#[proc_macro_derive(ProviderKindFromConfigTraitDerive)]
pub fn derive_provider_kind_from_config(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("cannot parse input into syn::DeriveInput"); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &Ident = &ast.ident;
    let data: Data = ast.data;
    let trait_name: Ident;
    let function_vec_idents: Vec<(Ident, ReturnType)>;
    //todo: how to match it without fs::read_to_string("./src/traits/provider_kind_from_config_trait.rs") ? import CONFIG type somehow?
    match fs::read_to_string("./src/traits/provider_kind_from_config_trait.rs") {
        Err(e) => panic!("file:  error: {e}"),
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
        let mut is_str = false;
        if let syn::ReturnType::Type(_, box_type) = &output {
            if let syn::Type::Reference(type_reference) = &**box_type {
                if let syn::Type::Path(reference_type_path) = &*type_reference.elem {
                    for i in &reference_type_path.path.segments {
                        if i.ident.to_string() == "str" {
                            is_str = true;
                        }
                    }
                }
            }
        }
        let variants_for_quote = variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            let config_field_name = syn::Ident::new(
                &format!(
                    "{}_{}",
                    function_name_ident
                        .to_string()
                        .to_case(Case::Snake)
                        .to_lowercase(),
                    variant_name.to_string().to_case(Case::Snake).to_lowercase()
                ),
                variant_name.span(),
            );
            if is_str {
                quote! {
                    #ident::#variant_name => &CONFIG.#config_field_name
                }
            } else {
                quote! {
                        #ident::#variant_name => CONFIG.#config_field_name
                }
            }
        });
        function_quote_vec_ident.push(quote! {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
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
