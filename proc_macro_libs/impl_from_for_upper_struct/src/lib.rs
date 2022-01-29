use proc_macro::TokenStream;
use quote::quote;

use syn;
use syn::Type;

/// struct and enum names must be like this
/// pub struct StructNameError {
///     pub source: Box<StructNameErrorEnum>,
/// }
/// #[derive(ImplFromForUpperStruct)]
/// pub enum StructNameErrorEnum {
///     One(One),
///     Two(Two),
/// }
#[proc_macro_derive(ImplFromForUpperStruct)]
pub fn derive_impl_from_for_upper_struct(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_impl_from_for_upper_struct syn::parse(input) failed");
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants,
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let ident = &ast.ident;
    let generated = variants.into_iter().map(|v| {
        let variant = v.ident;
        let inner_enum_type: Type;
        match &v.fields {
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() != 1 {
                    panic!(
                        "fields_unnamed.unnamed != 1, length is {}",
                        fields_unnamed.unnamed.len()
                    );
                }
                inner_enum_type = fields_unnamed.unnamed[0].ty.clone();
            }
            _ => panic!("v.fields is not syn::Fields::Unnamed"),
        }
        let string_ident = ident.to_string();
        let enum_handle = "Enum";
        match string_ident.find(enum_handle) {
            None => panic!(
                "Cannot find {} inside string variant name {}",
                enum_handle, string_ident
            ),
            Some(index) => {
                let struct_ident = syn::Ident::new(&&string_ident[..index], ident.span());
                quote! {
                    impl From<#inner_enum_type> for #struct_ident {
                        fn from(error: #inner_enum_type) -> Self {
                            #struct_ident {
                                source: Box::new(#ident::#variant(
                                    error,
                                )),
                                line: format!("{}:{}:{}", file!(), line!(), column!()),
                            }
                        }
                    }
                }
            }
        }
    });
    let gen = quote! {
        #(#generated)*
    };
    gen.into()
}
