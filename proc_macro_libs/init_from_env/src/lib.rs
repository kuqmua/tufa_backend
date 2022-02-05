use syn;
// use syn::Ident;
// use syn::Path;

use convert_case::Case;
use convert_case::Casing;

use proc_macro::TokenStream;

use quote::quote;
use syn::Ident;
use syn::LitStr;
use syn::Path;

#[proc_macro_derive(InitFromEnv)]
pub fn derive_init_from_env(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_init_from_env syn::parse(input) failed");
    let ident = &ast.ident;
    let error_ident = syn::Ident::new(&format!("{}Error", ident), ident.span());
    let error_enum_ident = syn::Ident::new(&format!("{}ErrorEnum", ident), ident.span());
    let generated_init_struct_fields = match ast.data.clone() {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
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
            });
            generated
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    let generated_functions = match ast.data.clone() {
        syn::Data::Struct(datastruct) => {
            let generated = datastruct.fields.into_iter().map(|field| {
                let enum_variant_ident_pascal_case: Ident;
                let enum_variant_ident_snake_case: Ident;
                match field.ident.clone() {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => {
                        enum_variant_ident_pascal_case = syn::Ident::new(
                            &format!("{}", field_ident).to_case(Case::Pascal),
                            ident.span(),
                        );
                        enum_variant_ident_snake_case =
                            syn::Ident::new(&format!("{}", field_ident), ident.span());
                    }
                };
                // let enum_variant_type_as_string: String;
                let enum_variant_type = match field.ty.clone() {
                    syn::Type::Path(type_path) => type_path.path,
                    _ => panic!("field.ty is not a syn::Type::Path!"),
                };
                let enum_variant_ident_value = syn::Ident::new(
                    &format!("{}_value", enum_variant_ident_snake_case),
                    ident.span(),
                );
                let env_var_name: Ident;
                let env_var_name_as_snake_case_string: LitStr;
                match field.ident {
                    None => panic!("field.ident is None"),
                    Some(field_ident) => {
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

                let env_var_name_as_screaming_snake_case_string =
                    syn::LitStr::new(&format!("{}", env_var_name), ident.span());

                println!("{}", env_var_name);
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
                                                    expected_env_var_type: String::from("test"),
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
                                        expected_env_var_type:  String::from("test"),
                                        file: "test",
                                        line: 32,
                                        column: 32,
                                    }
                                ),
                                was_dotenv_enable,
                            }
                        );},
                    }
                }
            });
            generated
        }
        _ => panic!("GenEnum only works on Struct"),
    };
    //
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
                // let enum_variant_type = match field.ty {
                //     syn::Type::Path(type_path) => type_path.path,
                //     _ => panic!("field.ty is not a syn::Type::Path!"),
                // };
                // ,
                //
                quote! {
                    #enum_variant_ident {
                        env_var_name: &'static str,
                        expected_env_var_type: String,
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
    // #(#generated_functions)*
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
    //     providers_link_parts_source: Resource::Local,
    // github_name: String::from("f"),
    // is_prints_enabled: true,
    // links_limit_twitter: 64,
    // error_red: 8,
    gen.into()
}

// fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError> {
//     let env_name = self.to_upper_snake_case();
// match std::env::var(&env_name) {
//     Ok(handle) => Ok(handle),
//     Err(e) => Err(ConfigError {
//         env_var_name_kind: ConfigEnvVarErrorType::#ident(*self),
//         was_dotenv_enable,
//         env_name,
//         // env_error: ConfigErrorInnerType::VarErrorHandle(e),
//     }),
// }
// }

// fn get_env_values_hashmap<T: std::str::FromStr>(was_dotenv_enable: bool) -> Result<HashMap<Self, T>, ConfigError> {
//     for env_var_name_kind in Self::iter() {
//         match env_var_name_kind.get_string_from_env_var(was_dotenv_enable) {
//             Ok(env_var_string) => match #ident::parse_string::<T>(env_var_string) {
//                 Ok(handle) => {
//                     hmap.insert(env_var_name_kind, handle);
//                 },
//                 Err(e) => {
//                             error_option = Some(ConfigError {
//                         env_var_name_kind: ConfigEnvVarErrorType::#ident(env_var_name_kind),
//                         was_dotenv_enable,
//                         env_name: env_var_name_kind.to_upper_snake_case(),
//                         // env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(
//                         //     VarOrIntParseError::Int(e),
//                         // ),
//                     });
//                     break;
//                 },
//             },
//             Err(e) => {
//                 error_option = Some(e);
//                 break;
//             }
//         }
//     }
//     Ok(hmap)
// }

// fn check_valid_typed_env_vars() {
//     if let Err(e) = dotenv() {
//         panic!("dotenv() failed, error: {:?}", e);
//     }
//     for env_var_name_kind in #ident::iter() {
//         match std::env::var(&env_var_name_kind.to_upper_snake_case()) {
//             Err(e) => panic!(
//                 "no such env name {} error: {:#?}",
//                 &env_var_name_kind.to_upper_snake_case(),
//                 e
//             ),
//             Ok(handle) => {
//                 if let Err(e) = #ident::parse_string::<#type_for_parsing>(handle) {
//                     panic!(
//                         "parse env var {} error: {:#?}",
//                         &env_var_name_kind.to_upper_snake_case(),
//                         e
//                     )
//                 }
//             }
//         }
//     }
// }

// fn check_compromised_typed_env_vars<T: std::str::FromStr>(was_dotenv_enable: bool){
//     match #ident::get_env_values_hashmap::<#type_for_parsing>(was_dotenv_enable) {
//         Err(e) => panic!("cannot get env values hashmap, error: {:#?}", e),
//         Ok(hashmap) => {
//             for (key, value) in hashmap {
//                 if value != #type_for_parsing::default() {
//                     panic!("{:?} is not assigned to default value", key);
//                 }
//             }
//         }
//     }
// }
