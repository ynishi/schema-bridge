use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Lit, Meta};

#[proc_macro_derive(SchemaBridge, attributes(schema_bridge, serde))]
pub fn derive_schema_bridge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let ts_impl = impl_to_ts(&input);
    let schema_impl = impl_to_schema(name, &input.data);

    // Check for string_conversion attribute
    let string_conversion = has_string_conversion(&input.attrs);

    let mut expanded = quote! {
        impl ::schema_bridge::SchemaBridge for #name {
            fn to_ts() -> String {
                #ts_impl
            }

            fn to_schema() -> ::schema_bridge::Schema {
                #schema_impl
            }
        }
    };

    // Generate Display and FromStr if requested
    if string_conversion {
        if let Data::Enum(_) = &input.data {
            let display_impl = impl_display(&input);
            let fromstr_impl = impl_fromstr(&input);

            expanded = quote! {
                #expanded

                #display_impl

                #fromstr_impl
            };
        }
    }

    TokenStream::from(expanded)
}

/// Check if #[schema_bridge(string_conversion)] attribute is present
fn has_string_conversion(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("schema_bridge") {
            if let Meta::List(meta_list) = &attr.meta {
                if let Ok(Meta::Path(path)) = syn::parse2(meta_list.tokens.clone()) {
                    if path.is_ident("string_conversion") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn impl_to_ts(input: &DeriveInput) -> proc_macro2::TokenStream {
    match &input.data {
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
                Fields::Unnamed(fields) => {
                    // Support for tuple structs, especially newtype pattern
                    if fields.unnamed.len() == 1 {
                        // Newtype pattern: delegate to the inner type
                        let inner_ty = &fields.unnamed[0].ty;
                        quote! {
                            <#inner_ty as ::schema_bridge::SchemaBridge>::to_ts()
                        }
                    } else {
                        // Multiple field tuple struct - represent as tuple
                        let field_types = fields.unnamed.iter().map(|f| {
                            let ty = &f.ty;
                            quote! {
                                <#ty as ::schema_bridge::SchemaBridge>::to_ts()
                            }
                        });

                        quote! {
                            let types = vec![#(#field_types),*];
                            format!("[{}]", types.join(", "))
                        }
                    }
                }
                Fields::Unit => quote! { "null".to_string() },
            }
        }
        Data::Enum(data) => {
            // Check for serde rename_all attribute
            let rename_all = get_serde_rename_all(&input.attrs);

            let variants = data.variants.iter().map(|v| {
                let variant_name = &v.ident;
                let variant_str = variant_name.to_string();

                // Apply rename_all transformation if present
                let ts_name = if let Some(ref rule) = rename_all {
                    apply_rename_rule(&variant_str, rule)
                } else {
                    variant_str
                };

                quote! {
                    format!("'{}'", #ts_name)
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

/// Extract rename_all from #[serde(rename_all = "...")]
fn get_serde_rename_all(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Meta::List(meta_list) = &attr.meta {
                // Parse the meta list
                let nested: Result<Meta, _> = syn::parse2(meta_list.tokens.clone());
                if let Ok(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("rename_all") {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Apply serde rename_all transformation
fn apply_rename_rule(name: &str, rule: &str) -> String {
    match rule {
        "lowercase" => name.to_lowercase(),
        "UPPERCASE" => name.to_uppercase(),
        "PascalCase" => name.to_string(), // Already PascalCase
        "camelCase" => {
            let mut chars = name.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_lowercase().chain(chars).collect(),
            }
        }
        "snake_case" => {
            let mut result = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() && i > 0 {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap());
            }
            result
        }
        "SCREAMING_SNAKE_CASE" => {
            let mut result = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() && i > 0 {
                    result.push('_');
                }
                result.push(ch.to_uppercase().next().unwrap());
            }
            result
        }
        "kebab-case" => {
            let mut result = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() && i > 0 {
                    result.push('-');
                }
                result.push(ch.to_lowercase().next().unwrap());
            }
            result
        }
        _ => name.to_string(), // Unknown rule, keep as-is
    }
}

fn impl_to_schema(_name: &Ident, _data: &Data) -> proc_macro2::TokenStream {
    // Placeholder for now, focusing on TS generation first
    quote! {
        ::schema_bridge::Schema::Any
    }
}

/// Generate Display implementation for enum
fn impl_display(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    if let Data::Enum(data) = &input.data {
        let rename_all = get_serde_rename_all(&input.attrs);

        let match_arms = data.variants.iter().map(|v| {
            let variant_name = &v.ident;
            let variant_str = variant_name.to_string();

            let display_str = if let Some(ref rule) = rename_all {
                apply_rename_rule(&variant_str, rule)
            } else {
                variant_str
            };

            quote! {
                #name::#variant_name => write!(f, "{}", #display_str)
            }
        });

        quote! {
            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        #(#match_arms),*
                    }
                }
            }
        }
    } else {
        quote! {}
    }
}

/// Generate FromStr implementation for enum
fn impl_fromstr(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    if let Data::Enum(data) = &input.data {
        let rename_all = get_serde_rename_all(&input.attrs);

        let match_arms = data.variants.iter().map(|v| {
            let variant_name = &v.ident;
            let variant_str = variant_name.to_string();

            let pattern_str = if let Some(ref rule) = rename_all {
                apply_rename_rule(&variant_str, rule)
            } else {
                variant_str
            };

            quote! {
                #pattern_str => ::std::result::Result::Ok(#name::#variant_name)
            }
        });

        quote! {
            impl ::std::str::FromStr for #name {
                type Err = String;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    match s {
                        #(#match_arms,)*
                        _ => ::std::result::Result::Err(format!("Unknown {}: {}", stringify!(#name), s))
                    }
                }
            }
        }
    } else {
        quote! {}
    }
}
