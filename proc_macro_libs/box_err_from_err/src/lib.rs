use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Ident};

#[proc_macro_derive(BoxErrFromErrDerive)]
pub fn derive_box_err_from_err(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    let ident = &ast.ident;
    let error_type_ident: Ident;
    match &ast.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Unnamed(unnamed_field) => {
                if unnamed_field.unnamed.len() != 1 {
                    panic!(
                        "unnamed_field.unnamed != 1, length is {}",
                        unnamed_field.unnamed.len()
                    );
                }
                match &unnamed_field.unnamed[0].ty {
                    syn::Type::Path(type_path) => {
                        if type_path.path.segments.len() != 1 {
                            panic!(
                                "type_path.path.segments != 1, length is {}",
                                type_path.path.segments.len()
                            );
                        }
                        match &type_path.path.segments[0].arguments {
                            syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                                if angle_bracketed_generic_arguments.args.len() != 1 {
                                    panic!(
                                        "angle_bracketed_generic_arguments.args != 1, length is {}",
                                        angle_bracketed_generic_arguments.args.len()
                                    );
                                }
                                match &angle_bracketed_generic_arguments.args[0] {
                                    syn::GenericArgument::Type(type_handle) => {
                                        match type_handle {
                                            syn::Type::Path(type_path) => {
                                                if type_path.path.segments.len() != 1 {
                                                    panic!(
                                                        "type_path.path.segments != 1, length is {}",
                                                        type_path.path.segments.len()
                                                    );
                                                }
                                                error_type_ident = type_path.path.segments[0].ident.clone();
                                            },
                                            _ => panic!("type_handle is not a syn::Type::Path!"),
                                        }
                                    },
                                    _ => panic!("generic_argument is not a syn::GenericArgument::Type!"),
                                }
                            },
                            _ => panic!("path_segment.arguments is not a syn::PathArguments::AngleBracketed!"),
                        }
                    }
                    _ => panic!("field.ty is not a syn::Type::Path!"),
                }
            }
            _ => panic!("data_struct.fields is not a syn::Fields::Unnamed!"),
        },
        _ => panic!("data is not a Struct!"),
    }
    let gen = quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "#write_stroke")
            }
        }

        impl From<#error_type_ident> for #ident {
            fn from(error: #error_type_ident) -> Self {
                #ident(Box::new(error))
            }
        }
    };
    gen.into()
}
