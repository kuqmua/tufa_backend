use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(GitInfoDerive)]
pub fn derive_git_info(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_git_info syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote! {
        impl GitInfo for #ident {
            fn get_git_info(&self) -> &GIT_INFO {
                &GIT_INFO
            }
        }
    };
    gen.into()
}
