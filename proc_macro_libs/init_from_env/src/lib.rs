use syn;
// use syn::Ident;
// use syn::Path;

use convert_case::Case;
use convert_case::Casing;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(InitFromEnv)]
pub fn derive_init_from_env(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_init_from_env syn::parse(input) failed");
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
    let enum_ident = syn::Ident::new(&format!("{}Enum", ident), ident.span());
    // #[derive(Debug)]
    let gen = quote! {
        pub enum #enum_ident {
            #(#generated)*
        }
    };
    gen.into()
}

// impl ConfigStruct {
//     pub fn new() -> Result<Self, ConfigError> {
//         let mut was_dotenv_enable = false;
//         if dotenv().is_ok() {
//             was_dotenv_enable = true;
//         }
//         let string_vars = EnvStringVar::get_env_values_hashmap::<String>(was_dotenv_enable)?;
//         let bool_vars = EnvBoolVar::get_env_values_hashmap::<bool>(was_dotenv_enable)?;
//         let handle_config: ConfigStruct = ConfigStruct {
//             github_name: string_vars[&EnvStringVar::GithubName].clone(),
//         };
//         ConfigStruct::wrap_config_checks(handle_config)
//     }
// }

// fn get_string_from_env_var(&self, was_dotenv_enable: bool) -> Result<String, ConfigError> {
//     let env_name = self.to_upper_snake_case();
//     match std::env::var(&env_name) {
//         Ok(handle) => Ok(handle),
//         Err(e) => Err(ConfigError {
//             env_var_name_kind: ConfigEnvVarErrorType::#ident(*self),
//             was_dotenv_enable,
//             env_name,
//             // env_error: ConfigErrorInnerType::VarErrorHandle(e),
//         }),
//     }
// }

// fn parse_string<T: std::str::FromStr>(value: String) -> Result<T, T::Err> {
//     match value.parse::<T>() {
//         Ok(handle) => Ok(handle),
//         Err(e) => Err(e),
//     }
// }

// fn get_env_values_hashmap<T: std::str::FromStr>(was_dotenv_enable: bool) -> Result<HashMap<Self, T>, ConfigError> {
//     let mut hmap: HashMap<Self, T> = HashMap::new();
//     let mut error_option: Option<ConfigError> = None;
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
//     if let Some(error) = error_option {
//         return Err(error);
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
