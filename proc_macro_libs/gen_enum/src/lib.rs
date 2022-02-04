use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GenEnum)]
pub fn derive_gen_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_gen_enum syn::parse(input) failed");
    let len = match ast.data.clone() {
        syn::Data::Struct(_) => {
            // println!("ast {:#?}", ast);
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    let ident = &ast.ident;
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
