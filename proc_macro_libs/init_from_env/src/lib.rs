use syn;
use syn::Ident;
use syn::LitStr;
use syn::Path;

use convert_case::Case;
use convert_case::Casing;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(InitFromEnv)]
pub fn derive_init_from_env(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_init_from_env syn::parse(input) failed");
    let ident = &ast.ident;
    let error_ident = syn::Ident::new(&format!("{}Error", ident), ident.span());
    let error_enum_ident = syn::Ident::new(&format!("{}ErrorEnum", ident), ident.span());
    let generated_init_struct_fields = match ast.data.clone() {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident: Ident;
            let enum_variant_ident_value: Ident;
            match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => {
                    enum_variant_ident = field_ident.clone();
                    enum_variant_ident_value =
                        syn::Ident::new(&format!("{}_value", field_ident), ident.span());
                }
            };
            quote! {
                #enum_variant_ident: #enum_variant_ident_value,
            }
        }),
        _ => panic!("InitFromEnv only works on Struct"),
    };
    let generated_functions = match ast.data.clone() {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let enum_variant_ident_pascal_case: Ident;
            let enum_variant_ident_value: Ident;
            let env_var_name: Ident;
            let env_var_name_as_snake_case_string: LitStr;
            match field.ident.clone() {
                None => panic!("field.ident is None"),
                Some(field_ident) => {
                    enum_variant_ident_pascal_case = syn::Ident::new(
                        &format!("{}", field_ident).to_case(Case::Pascal),
                        ident.span(),
                    );
                    enum_variant_ident_value =
                        syn::Ident::new(&format!("{}_value", field_ident), ident.span());
                    env_var_name = syn::Ident::new(
                        &format!("{}", field_ident)
                            .to_case(Case::Snake)
                            .to_uppercase(),
                        ident.span(),
                    );
                    env_var_name_as_snake_case_string =
                        syn::LitStr::new(&format!("{}", field_ident), ident.span());
                }
            };
            let enum_variant_type: Path;
            let enum_variant_type_as_string: LitStr;
            match field.ty.clone() {
                syn::Type::Path(type_path) => {
                    enum_variant_type = type_path.path.clone();
                    //todo: remove :? later
                    enum_variant_type_as_string =
                        syn::LitStr::new(&format!("{:?}", type_path), ident.span());
                }
                _ => panic!("field.ty is not a syn::Type::Path!"),
            };
            let env_var_name_as_screaming_snake_case_string =
                syn::LitStr::new(&format!("{}", env_var_name), ident.span());
            quote! {
                let #enum_variant_ident_value: #enum_variant_type;
                match std::env::var(#env_var_name_as_screaming_snake_case_string) {
                    Ok(string_handle) => {
                        match string_handle.parse::<#enum_variant_type>() {
                            Err(e) => {
                                return Err(
                                    #error_ident {
                                        source: Box::new(
                                            #error_enum_ident::#enum_variant_ident_pascal_case {
                                                env_var_name: #env_var_name_as_snake_case_string,
                                                expected_env_var_type: #enum_variant_type_as_string,
                                                file: "test",
                                                line: 32,
                                                column: 32,
                                            }
                                        ),
                                        was_dotenv_enable,
                                    }
                                );
                            },
                            Ok(handle) => {
                                #enum_variant_ident_value = handle;
                            },
                        }
                    },
                    Err(e) => {
                        return Err(
                            #error_ident {
                                source: Box::new(
                                    #error_enum_ident::#enum_variant_ident_pascal_case {
                                        env_var_name: #env_var_name_as_snake_case_string,
                                        expected_env_var_type: #enum_variant_type_as_string,
                                        file: "test",
                                        line: 32,
                                        column: 32,
                                    }
                                ),
                                was_dotenv_enable,
                            }
                        );
                    },
                }
            }
        }),
        _ => panic!("GenEnum only works on Struct"),
    };
    let generated_enum_error_variants = match ast.data {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
                let enum_variant_ident = match field.ident.clone() {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => syn::Ident::new(
                        &format!("{}", field_ident).to_case(Case::Pascal),
                        ident.span(),
                    ),
                };
                quote! {
                    #enum_variant_ident {
                        env_var_name: &'static str,
                        expected_env_var_type: &'static str,
                        file: &'static str,
                        line: u32,
                        column: u32,
                    },
                }
            });
            generated
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    let gen = quote! {
        pub struct #error_ident {
            pub source: Box<#error_enum_ident>,
            pub was_dotenv_enable: bool,
        }
        pub enum #error_enum_ident {
            #(#generated_enum_error_variants)*
        }
        impl #ident {
            fn new() -> Result<Self, #error_ident> {
                let was_dotenv_enable = dotenv().is_ok();
                #(#generated_functions)*
                Ok(
                    Self {
                        #(#generated_init_struct_fields)*
                    }
                )
            }
        }
    };
    gen.into()
}
