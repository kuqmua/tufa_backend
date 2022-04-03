use convert_case::Case;
use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GenEnumWithoutValuesDerive)]
pub fn derive_gen_enum_without_values(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_gen_enum_without_values syn::parse(input) failed");
    let ident = &ast.ident;
    let generated = match ast.data {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
                let enum_variant_ident = match field.ident {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{}", field_ident).to_case(Case::Pascal),
                        ident.span(),
                    ),
                };
                quote! {
                    #enum_variant_ident,
                }
            });
            generated
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    let enum_ident = syn::Ident::new(&format!("{}EnumWithoutValues", ident), ident.span());
    let gen = quote! {
         #[derive(Debug, strum_macros::Display, EnumIter, EnumExtenstion)]
        pub enum #enum_ident {
            #(#generated)*
        }
    };
    gen.into()
}
