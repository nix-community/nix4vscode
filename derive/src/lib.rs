use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

macro_rules! parse_meta {
    ($attr:ident as $ty:ty) => {
        match darling::ast::NestedMeta::parse_meta_list($attr.into()) {
            Ok(v) => match <$ty as darling::FromMeta>::from_list(&v) {
                Ok(v) => v,
                Err(e) => {
                    return proc_macro::TokenStream::from(e.write_errors());
                }
            },
            Err(e) => {
                return proc_macro::TokenStream::from(darling::Error::from(e).write_errors());
            }
        }
    };
    ($attr:ident) => {
        parse_meta!($attr as _)
    };
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct ApiAttr {
    nodefault: bool,
}

impl ApiAttr {
    fn attach_struct(&self, input: &mut syn::ItemStruct) {
        input.attrs.push(parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        });
        if !self.nodefault {
            input.attrs.push(parse_quote! {
                #[derive(Default)]
            });
        }
        input.attrs.push(parse_quote! {
            #[serde(default)]
        });
        input.attrs.push(parse_quote! {
            #[serde(rename_all = "camelCase")]
        });

        input.fields.iter_mut().for_each(|item| {
            item.vis = syn::parse_str("pub").unwrap();
        })
    }

    fn attach_enum(&self, input: &mut syn::ItemEnum) {
        input.attrs.push(parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        });
        if self.nodefault {
            input.attrs.push(parse_quote! {
                #[derive(Default, Copy)]
            });
        }
        input.attrs.push(parse_quote! {
            #[serde(rename_all = "camelCase")]
        });
    }
}

#[proc_macro_attribute]
pub fn api(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_meta!(attr as ApiAttr);
    if let Ok(mut input) = syn::parse::<syn::ItemStruct>(input.clone()) {
        attr.attach_struct(&mut input);
        quote! {
            #input
        }
        .into()
    } else {
        let mut input = parse_macro_input!(input as syn::ItemEnum);
        attr.attach_enum(&mut input);
        quote! {
            #input
        }
        .into()
    }
}
