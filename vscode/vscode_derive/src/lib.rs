use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, parse_quote,
};

#[derive(Debug, Default)]
struct ApiAttr {
    default: bool,
}

impl Parse for ApiAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut res = Self::default();
        let id: syn::Ident = input.parse()?;

        if id.to_string().to_lowercase() == "default" {
            res.default = true;
        }

        Ok(res)
    }
}

impl ApiAttr {
    fn attach_struct(&self, input: &mut syn::ItemStruct) {
        input.attrs.push(parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        });
        if self.default {
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
    }

    fn attach_enum(&self, input: &mut syn::ItemEnum) {
        input.attrs.push(parse_quote! {
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        });
        if self.default {
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
    let attr = Parser::parse2(ApiAttr::parse, attr.into()).unwrap_or_default();
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
