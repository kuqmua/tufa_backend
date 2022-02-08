use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnumExtenstion)]
pub fn derive_enum_extension(input: TokenStream) -> TokenStream {
    //it only supported for enums without values
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    //todo to implement into_array() and into_vec - must implement Default for all inner variant types
    let len = match ast.data.clone() {
        syn::Data::Enum(enum_item) => {
            // println!("{:#?}", enum_item.variants);
            enum_item.variants.len()
        },
        _ => panic!("EnumVariantCount only works on Enums"),
    };
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| {
            let variant_ident = v.ident;
            match v.fields {
                syn::Fields::Named(_) => todo!(),
                syn::Fields::Unnamed(_) => quote! { #variant_ident(Default::default()) },
                syn::Fields::Unit => quote! { #variant_ident, },
            }
        }),
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            fn get_length() -> usize {
                #len
            }
            fn into_array() -> [#name; #len] {
                [ #(#name::#variants),* ]
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
            fn to_lower_snake_case(&self) -> String {
                format!("{:?}", self).to_case(Case::Snake).to_lowercase()
            }
        }
    };
    gen.into()
}
