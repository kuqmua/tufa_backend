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

#[proc_macro_derive(EnumIntoArray)]
pub fn derive_enum_into_array(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();
    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| v.ident),
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let enum_name = syn_item.ident;
    let expanded = quote! {
        impl #enum_name {
            pub fn into_array() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants),* ]
            }
        }
    };
    expanded.into()
}

#[proc_macro_derive(EnumExtenstion)]
pub fn derive_enum_extension(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name= &ast.ident;
    let gen = quote! {
        impl EnumExtenstion for #name {
            #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
            fn into_array() -> &'static [Self] {
                Self::into_array()
            }
            #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
            fn into_vec() -> Vec<Self> {
                let mut self_vec = Vec::with_capacity(Self::get_length());
                for self_variant in Self::iter() {
                    self_vec.push(self_variant);
                }
                self_vec
            }
            #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
            fn into_string_name_and_variant_hashmap() -> HashMap<String, Self> {
                let mut variants_hashmap: HashMap<String, Self> =
                    HashMap::with_capacity(Self::get_length());
                for variant in Self::iter() {
                    variants_hashmap.insert(format!("{}", variant), variant);
                }
                variants_hashmap
            }
            #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
            fn into_string_name_and_variant_tuple_vec() -> Vec<(String, Self)> {
                let mut variants_vec = Vec::with_capacity(Self::get_length());
                for variant in Self::iter() {
                    variants_vec.push((format!("{}", variant), variant));
                }
                variants_vec
            }
            fn to_upper_snake_case(&self) -> String {
                format!("{:?}", self).to_case(Case::Snake).to_uppercase()
            }
        }
    };
    gen.into()
}