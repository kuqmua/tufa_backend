use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let len = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let expanded = quote! {
    const ENUM_LENGTH: usize = #len;
        };
    expanded.into()
}

#[proc_macro_derive(AllVariants)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();

    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| v.ident),
        _ => panic!("AllVariants only works on enums"),
    };
    let enum_name = syn_item.ident;

    let expanded = quote! {
        impl #enum_name {
            pub fn all_variants() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants),* ]
            }
        }
    };
    expanded.into()
}
