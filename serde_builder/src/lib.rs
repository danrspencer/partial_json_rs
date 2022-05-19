use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, AttributeArgs, GenericArgument, ItemStruct, Lit, Meta, NestedMeta,
    PathArguments, PathSegment, ReturnType, Type,
};

#[proc_macro_derive(SerdeBuilder)]
/// Generate a Serde builder for a struct
/// ```
/// # use serde_builder::SerdeBuilder;
///
/// #[derive(SerdeBuilder)]
/// struct Foo {
///    bar: String,
/// }
///
/// Foo::serde_builder();
pub fn derive_serde_builder(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);

    let struct_name = item.ident.clone();
    let builder_name = format_ident!("{}SerdeBuilder", item.ident);

    let fields = item
        .fields
        .into_iter()
        .map(|f| quote! { #f.ident: Option<#f.ty> })
        .collect::<Vec<_>>();

    // println!(
    //     "{}",
    //     item.fields
    //         .into_iter()
    //         .map(|f| (f.ident, f.ty))
    //         .collect::<Vec<_>>()
    // );

    let expanded = quote! {
        impl #struct_name {
            pub fn serde_builder() -> #builder_name {
                #builder_name::new()
            }
        }

        // #[derive(serde::Serialize, serde::Deserialize)]
        pub struct #builder_name {
            #(#fields,)*
        }

        impl #builder_name {
            pub fn new() -> Self {
                Self {
                }
            }
        }
    };

    print!("{}", expanded);
    TokenStream::from(expanded)
}
