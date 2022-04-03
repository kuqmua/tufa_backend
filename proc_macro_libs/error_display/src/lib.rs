use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ErrorDisplay)]
pub fn derive_error_display(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_error_display syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = match ast.data {
        syn::Data::Struct(_) => quote! {
            if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled {
                write!(
                    f,
                    "{}\n{}\n{}",
                    self.where_was.source_place_with_readable_time(),
                    self.where_was.github_source_place_with_readable_time(),
                    self.source
                )
            } else if CONFIG.is_show_source_place_enabled {
                write!(
                    f,
                    "{}\n{}",
                    self.where_was.source_place_with_readable_time(),
                    self.source
                )
            } else if CONFIG.is_show_github_source_place_enabled {
                write!(
                    f,
                    "{}\n{}",
                    self.where_was.github_source_place_with_readable_time(),
                    self.source
                )
            } else {
                write!(f, "{}", self.source)
            }
        },
        syn::Data::Enum(data_enum) => {
            let generated = data_enum.variants.into_iter().map(|v| {
                let variant = v.ident;
                quote! {
                    #ident::#variant { source, where_was } => {
                        if CONFIG.is_show_source_place_enabled && CONFIG.is_show_github_source_place_enabled
                        {
                            write!(
                                f,
                                "{}\n{}\n{}",
                                where_was.source_place_with_readable_time(),
                                where_was.github_source_place_with_readable_time(),
                                source
                            )
                        } else if CONFIG.is_show_source_place_enabled {
                            write!(
                                f,
                                "{}\n{}",
                                where_was.source_place_with_readable_time(),
                                source
                            )
                        } else if CONFIG.is_show_github_source_place_enabled {
                            write!(
                                f,
                                "{}\n{}",
                                where_was.github_source_place_with_readable_time(),
                                source
                            )
                        } else {
                            write!(f, "{}", source)
                        }
                    }
                }
            });
            quote! {
                match self {
                    #(#generated)*
                }
            }
        }
        _ => panic!("ErrorDisplay only works on enum and struct"),
    };
    quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                #gen
            }
        }
    }
    .into()
}
