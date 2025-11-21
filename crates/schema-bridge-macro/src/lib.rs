use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(SchemaBridge, attributes(serde))]
pub fn derive_schema_bridge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let ts_impl = impl_to_ts(&name, &input.data);
    let schema_impl = impl_to_schema(&name, &input.data);

    let expanded = quote! {
        impl ::schema_bridge::SchemaBridge for #name {
            fn to_ts() -> String {
                #ts_impl
            }

            fn to_schema() -> ::schema_bridge::Schema {
                #schema_impl
            }
        }
    };

    TokenStream::from(expanded)
}

fn impl_to_ts(_name: &Ident, data: &Data) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let fields_ts = fields.named.iter().map(|f| {
                        let field_name = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            format!("{}: {};", stringify!(#field_name), <#ty as ::schema_bridge::SchemaBridge>::to_ts())
                        }
                    });

                    quote! {
                        let fields = vec![#(#fields_ts),*];
                        format!("{{ {} }}", fields.join(" "))
                    }
                }
                _ => quote! { "any".to_string() }, // Tuple structs not supported yet
            }
        }
        Data::Enum(data) => {
            let variants = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                // Simple enum for now
                quote! {
                    format!("'{}'", stringify!(#variant_name))
                }
            });

            quote! {
                let variants = vec![#(#variants),*];
                variants.join(" | ")
            }
        }
        _ => quote! { "any".to_string() },
    }
}

fn impl_to_schema(_name: &Ident, _data: &Data) -> proc_macro2::TokenStream {
    // Placeholder for now, focusing on TS generation first
    quote! {
        ::schema_bridge::Schema::Any
    }
}
