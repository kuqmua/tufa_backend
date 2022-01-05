use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ImplDisplayDerive)]
pub fn derive_impl_display(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    let ident = &ast.ident;
    let gen = quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self.source)
            }
        }
    };
    gen.into()
}
