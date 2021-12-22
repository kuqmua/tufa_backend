use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(EnvVarTypedTrait)]
pub fn derive_env_var_typed(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name= &ast.ident;
    let gen = quote! {
        impl EnvVarTypedTrait for #name {
            fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError> {
                let string_name = self.to_upper_snake_case();
                match std::env::var(&string_name) {
                    Ok(handle) => Ok(handle),
                    Err(e) => Err(ConfigError {
                        env_var_name_kind: ConfigEnvVarErrorType::#name(*self),
                        was_dotenv_enable,
                        env_name: string_name,
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
        
            fn get_env_values_hashmap<T: std::str::FromStr>() -> Result<HashMap<Self, T>, ConfigError> {
                let was_dotenv_enable: bool;
                match dotenv() {
                    Ok(_) => {
                        was_dotenv_enable = true;
                    }
                    Err(e) => {
                        was_dotenv_enable = false;
                        println!(
                            "dotenv() failed, trying without {} error: {:?}",
                            ENV_FILE_NAME, e
                        );
                    }
                }
                let mut hmap: HashMap<Self, T> = HashMap::new();
                let mut error_option: Option<ConfigError> = None;
                for env_var_name_kind in Self::iter() {
                    match env_var_name_kind.get_string_from_env_var(was_dotenv_enable) {
                        Ok(env_var_string) => match #name::parse_string::<T>(env_var_string) {
                            Ok(handle) => {
                                hmap.insert(env_var_name_kind, handle);
                            },
                            Err(e) => {
                                        error_option = Some(ConfigError {
                                    env_var_name_kind: ConfigEnvVarErrorType::#name(env_var_name_kind),
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
        }
    };
    gen.into()
}