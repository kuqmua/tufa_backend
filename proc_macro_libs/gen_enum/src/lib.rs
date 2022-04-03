use convert_case::Case;
use convert_case::Casing;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(GenEnumDerive)]
pub fn derive_gen_enum(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_gen_enum syn::parse(input) failed");
    let ident = &ast.ident;
    let generated = match ast.data {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
                let enum_variant_ident = match field.ident {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{field_ident}").to_case(Case::Pascal),
                        ident.span(),
                    ),
                };
                let enum_variant_type = match field.ty {
                    syn::Type::Path(type_path) => type_path.path,
                    _ => panic!("field.ty is not a syn::Type::Path!"),
                };

                quote! {
                    #enum_variant_ident(#enum_variant_type),
                }
            });
            generated
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    let enum_ident = syn::Ident::new(&format!("{ident}Enum"), ident.span());
    let gen = quote! {
        #[derive(Debug, EnumIter, strum_macros::Display, EnumExtenstion)]
        pub enum #enum_ident {
            #(#generated)*
        }
    };
    gen.into()
}
