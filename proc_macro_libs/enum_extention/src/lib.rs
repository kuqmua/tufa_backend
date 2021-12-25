use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnumExtenstion)]
pub fn derive_enum_extension(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    let len = match ast.data.clone() {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| v.ident),
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let name = &ast.ident;
    let gen = quote! {
        impl EnumExtenstion for #name {
            fn get_length() -> usize {
                #len
            }
            fn into_array() -> &'static[#name] {
                &[ #(#name::#variants),* ]
            }
            fn into_vec() -> Vec<Self> {
                let mut self_vec = Vec::with_capacity(Self::get_length());
                for self_variant in Self::iter() {
                    self_vec.push(self_variant);
                }
                self_vec
            }
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
