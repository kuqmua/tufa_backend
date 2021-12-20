extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumExtenstion)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}


fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    // impl EnumExtenstion for #name {
    //     fn hello_world() {
    //         println!("Hello, World! My name is {}", stringify!(#name));
    //     }
    // }
    quote! {
        impl EnumExtenstion for #name {
            #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
            fn into_array() -> &'static [Self] {
                Self::all_variants()
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

    }
}

