use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(GitInfoDerive)]
pub fn derive_git_info(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_git_info syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote! {
        impl GitInfo for #ident {
            #[deny(
                clippy::indexing_slicing,
                clippy::unwrap_used,
                clippy::integer_arithmetic,
                clippy::float_arithmetic
            )]
            fn git_info(&self) -> String {
                GIT_INFO.get_git_commit_string()
            }
        }
    };
    gen.into()
}
