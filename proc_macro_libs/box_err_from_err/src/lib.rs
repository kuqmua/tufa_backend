use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BoxErrFromErrDerive)]
pub fn derive_box_err_from_err(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    let ident = &ast.ident;
    // let string_ident = ident.to_string();
    // let write_stroke = syn::Ident::new(&string_ident, ident.span());
    // let error_name_ident = syn::Ident::new(error_name, ident.span());s
    let gen = quote! {
        impl fmt::Display for #ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "#write_stroke")
            }
        }

        impl From<ConnectionError> for #ident {
            fn from(error: ConnectionError) -> Self {
                #ident(Box::new(error))
            }
        }
    };
    gen.into()
}
