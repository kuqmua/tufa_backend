////////////////////////////////////////////////////////////////
// use proc_macro;
use syn;
// #[macro_use]
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumVariantCount)]
// #[some_attribute]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let len = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let expanded = quote! {
    const LENGTH: usize = #len;
        };
    expanded.into()
}

// // #[derive(EnumVariantCount)]
// pub enum SomeThing {
//     Arxiv,
//     Biorxiv,
//     Github,
// }
////////////////////////////////
