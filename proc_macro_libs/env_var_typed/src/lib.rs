use crate::syn::Ident;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnvVarTypedTrait)]
pub fn derive_env_var_typed(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let ident: &Ident = &ast.ident;
    let u8_var_name = "EnvU8Var";
    let i64_var_name = "EnvI64Var";
    let bool_var_name = "EnvBoolVar";
    let string_var_name = "EnvStringVar";
    let type_for_parsing: Ident;
    if ident == u8_var_name {
        type_for_parsing = syn::Ident::new("u8", ident.span());
    } else if ident == i64_var_name {
        type_for_parsing = syn::Ident::new("i64", ident.span());
    } else if ident == bool_var_name {
        type_for_parsing = syn::Ident::new("bool", ident.span());
    } else if ident == string_var_name {
        type_for_parsing = syn::Ident::new("String", ident.span());
    } else {
        panic!("Unexpected ident: {:#?}", ident);
    }
    let gen = quote! {
        impl EnvVarTypedTrait for #ident {
            fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError> {
                let env_name = self.to_upper_snake_case();
                match std::env::var(&env_name) {
                    Ok(handle) => Ok(handle),
                    Err(e) => Err(ConfigError {
                        env_var_name_kind: ConfigEnvVarErrorType::#ident(*self),
                        was_dotenv_enable,
                        env_name,
                        // env_error: ConfigErrorInnerType::VarErrorHandle(e),
                    }),
                }
            }

            fn parse_string<T: std::str::FromStr>(value: String) -> Result<T, T::Err> {
                match value.parse::<T>() {
                    Ok(handle) => Ok(handle),
                    Err(e) => Err(e),
                }
            }

            fn get_env_values_hashmap<T: std::str::FromStr>(was_dotenv_enable: bool) -> Result<HashMap<Self, T>, ConfigError> {
                let mut hmap: HashMap<Self, T> = HashMap::new();
                let mut error_option: Option<ConfigError> = None;
                for env_var_name_kind in Self::iter() {
                    match env_var_name_kind.get_string_from_env_var(was_dotenv_enable) {
                        Ok(env_var_string) => match #ident::parse_string::<T>(env_var_string) {
                            Ok(handle) => {
                                hmap.insert(env_var_name_kind, handle);
                            },
                            Err(e) => {
                                        error_option = Some(ConfigError {
                                    env_var_name_kind: ConfigEnvVarErrorType::#ident(env_var_name_kind),
                                    was_dotenv_enable,
                                    env_name: env_var_name_kind.to_upper_snake_case(),
                                    // env_error: ConfigErrorInnerType::VarOrIntParseErrorErrorHandle(
                                    //     VarOrIntParseError::Int(e),
                                    // ),
                                });
                                break;
                            },
                        },
                        Err(e) => {
                            error_option = Some(e);
                            break;
                        }
                    }
                }
                if let Some(error) = error_option {
                    return Err(error);
                }
                Ok(hmap)
            }

            fn check_valid_typed_env_vars() {
                if let Err(e) = dotenv() {
                    panic!("dotenv() failed, error: {:?}", e);
                }
                for env_var_name_kind in #ident::iter() {
                    match std::env::var(&env_var_name_kind.to_upper_snake_case()) {
                        Err(e) => panic!(
                            "no such env name {} error: {:#?}",
                            &env_var_name_kind.to_upper_snake_case(),
                            e
                        ),
                        Ok(handle) => {
                            if let Err(e) = #ident::parse_string::<#type_for_parsing>(handle) {
                                panic!(
                                    "parse env var {} error: {:#?}",
                                    &env_var_name_kind.to_upper_snake_case(),
                                    e
                                )
                            }
                        }
                    }
                }
            }
            fn check_compromised_typed_env_vars<T: std::str::FromStr>(was_dotenv_enable: bool){
                match #ident::get_env_values_hashmap::<#type_for_parsing>(was_dotenv_enable) {
                    Err(e) => panic!("cannot get env values hashmap, error: {:#?}", e),
                    Ok(hashmap) => {
                        for (key, value) in hashmap {
                            if value != #type_for_parsing::default() {
                                panic!("{:?} is not assigned to default value", key);
                            }
                        }
                    }
                }
            }
        }
    };
    gen.into()
}
