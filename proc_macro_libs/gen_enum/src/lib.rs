use proc_macro::TokenStream;
use quote::quote;
use syn;

use convert_case::Case;
use convert_case::Casing;

#[proc_macro_derive(GenEnum)]
pub fn derive_gen_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_gen_enum syn::parse(input) failed");
    let ident = &ast.ident;
    let len = match ast.data {
        syn::Data::Struct(datastruct) => {
            for field in datastruct.fields {
                match field.ident {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => {
                        let enum_variant_ident = syn::Ident::new(
                            &format!("{}", field_ident).to_case(Case::Pascal),
                            ident.span(),
                        );
                        println!("enum_variant_ident {:#?}", enum_variant_ident);
                    }
                }
            }
        }
        _ => panic!("GenEnum only works on Struct"),
    };

    println!("/////");
    println!("{}", ident);
    println!("/////");
    let enum_ident = syn::Ident::new(&format!("{}Enum", ident), ident.span());
    println!("{}", enum_ident);
    // #[derive(Debug)]
    let gen = quote! {
        pub enum #enum_ident {
            One,
            Two
        }
    };
    gen.into()
}

// impl EnumExtenstion for #name {
//     fn get_length() -> usize {
//         #len
//     }
// }
