use proc_macro::TokenStream;
use quote::quote;
use syn::{ meta::ParseNestedMeta,  parse::Parse, parse_macro_input, Data, DeriveInput, Field, Ident, LitStr, Meta };

struct Conversion {
    source_field: Ident,
    target_field: Option<Ident>,
}

#[proc_macro_derive(GenFrom, attributes(into))]
pub fn gen_from_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut target_type: Option<Ident> = None;
    let mut conversions: Vec<Conversion> = vec![];

    for attr in &input.attrs {
        if !attr.path().is_ident("into") {
            continue;
        } 

        if target_type.is_some() {
            panic!("[Parser]: Target type is already specified.");
        }

        let target: Ident = attr.parse_args().inspect_err(|err| {
            panic!("[Parser]: Can't parse `into` identity (struct-level) | {}", err);
        }).unwrap();
        target_type = Some(target);
    }

    if target_type.is_none() {
        panic!("[Parser]: No target type specified.");
    }

    // Ensure the data is a struct
    if let Data::Struct(data) = &input.data {
        for field in data.fields.iter() {
            parse_field(field, &mut conversions);
        }
    };

    let target = target_type.expect("Target type must be specified via `#[into(...)]` attribute.");

    let conversion_code: Vec<_> = conversions.iter().map(|conv| {
        let source = &conv.source_field;

        if let Some(target_field) = &conv.target_field {
            quote! {
                #target_field: value.#source.into(),
            }
        }
        else {
            quote! {
                #source: value.#source.into(),
            }
        }

    }).collect();

    let expanded = quote! {

        impl From<#name> for #target {
            fn from(value: #name) -> #target {
                #target {
                    #(#conversion_code)*
                }
            }
        }

    };

    #[cfg(feature = "show-expansion")]
    println!("EXPANSION: \n{}", expanded);

    TokenStream::from(expanded)
}


/// Parse data structure field
fn parse_field(field: &Field, conversions: &mut Vec<Conversion>) {
    let name: Ident = field.ident.clone().unwrap();
    let mut conv_ident: Option<Ident> = None;

    let mut skip: bool = false;

    for attr in &field.attrs {
        if attr.path().is_ident("into") {
            match &attr.meta {
                Meta::List(list) => {
                    let _ = list.parse_nested_meta(|meta| {

                        if let Some(n) = parse_name(&meta) {
                            conv_ident = Some(n);
                        }

                        if meta.path.is_ident("skip") {
                            println!("Skipping field...");
                            skip = true;
                        };

                        Ok(())
                    }).inspect_err(|err| { 
                        panic!("[Parse field error]: {}", err);
                    });
                },
                Meta::Path(_) => {},
                Meta::NameValue(_) => {},
            }
        }
    }

    if !skip {
        conversions.push(Conversion { source_field: name, target_field: conv_ident });
    }
}


/// Parse name
/// 
/// # Example
/// #[into(name="v1")]
/// 
fn parse_name(meta: &ParseNestedMeta) -> Option<Ident>
{
    // #[into(name="v1", ...)]
    if meta.path.is_ident("name") {
        let value = meta.value().expect("No field name specified");
        let v: LitStr = value.parse().expect("Can't parse name");
        let ident = v.parse_with(syn::Ident::parse).expect("Can't parse identity name");
        return Some(ident)
    }

    None
}