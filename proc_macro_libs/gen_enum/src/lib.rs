use proc_macro::TokenStream;
use quote::quote;
use syn;

use convert_case::Case;
use convert_case::Casing;
use syn::Ident;
use syn::Path;

#[proc_macro_derive(GenEnum)]
pub fn derive_gen_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_gen_enum syn::parse(input) failed");
    let ident = &ast.ident;
    // println!("ast ddta {:#?}", ast.data);
    let generated = match ast.data {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
                let enum_variant_ident: Ident;
                let enum_variant_type: Path;
                match field.ident {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => {
                        enum_variant_ident = syn::Ident::new(
                            &format!("{}", field_ident).to_case(Case::Pascal),
                            ident.span(),
                        );
                    }
                }
                match field.ty {
                    syn::Type::Path(type_path) => {
                        enum_variant_type = type_path.path;
                    }
                    _ => panic!("field.ty is not a syn::Type::Path!"),
                }
                println!("enum_variant_ident {:#?}", enum_variant_ident);
                quote! {
                    #enum_variant_ident(#enum_variant_type),
                }
            });
            // println!("GEN {:#?}", generated);
            generated
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
            #(#generated)*
        }
    };
    gen.into()
}

// impl EnumExtenstion for #name {
//     fn get_length() -> usize {
//         #len
//     }
// }
