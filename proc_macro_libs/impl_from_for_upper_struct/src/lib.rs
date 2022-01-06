use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ImplFromForUpperStruct)]
pub fn derive_impl_from_for_upper_struct(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("derive_enum_extension syn::parse(input) failed");
    let ident = &ast.ident;
    let variants = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants,
        _ => panic!("EnumIntoArray only works on enums"),
    };
    let generated = variants.into_iter().map(|v| {
        let variant = v.ident;
        let string_variant = variant.to_string();
        let enum_handle = "Enum";
        match string_variant.find(enum_handle) {
            None => panic!(
                "Cannot find {} inside string variant name {}",
                enum_handle, string_variant
            ),
            Some(index) => {
                let struct_name = &string_variant[..index];
                let struct_ident = syn::Ident::new(&struct_name, ident.span());
                quote! {
                    impl From<#variant> for #struct_ident {
                        fn from(error: #variant) -> Self {
                            #struct_ident {
                                source: Box::new(#ident::#variant(
                                    error,
                                )),
                            }
                        }
                    }
                }
            }
        }
    });
    let gen = quote! {
        #(#generated)*
    };
    gen.into()
}
