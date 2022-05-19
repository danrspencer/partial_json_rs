use proc_macro::TokenStream;
use syn::{
    parse_macro_input, AttributeArgs, GenericArgument, ItemStruct, Lit, Meta, NestedMeta,
    PathArguments, PathSegment, ReturnType, Type,
};

#[proc_macro_attribute]
/// Generate a Serde builder for a struct
/// ```
/// # use serde_builder::serde_builder;
///
/// #[serde_builder]
/// struct Foo {
///    bar: String,
/// }
///
/// Foo::serde_builder();
pub fn serde_builder(_args: TokenStream, item: TokenStream) -> TokenStream {
    let raw_item = item.clone();
    let item = parse_macro_input!(item as ItemStruct);

    let builder_name = format!("{}SerdeBuilder", item.ident);

    format!(
        r#"
          {original_struct}

          impl {struct_name} {{
              pub fn serde_builder() -> {builder_name} {{
                {builder_name}::new()
              }}
          }}

          struct {builder_name} {{
              
          }}

          impl {builder_name} {{
            pub fn new() -> Self {{
                Self {{
                
                }}
            }}
          }}
    "#,
        original_struct = raw_item,
        struct_name = item.ident,
        builder_name = builder_name,
    )
    .parse()
    .expect("Generated invalid tokens")
}
