use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Attribute, AttrStyle, MacroDelimiter, Meta, MetaList, Path, PathSegment, token};
use syn::punctuated::Punctuated;
use syn::token::Pound;

pub(crate) fn create_derive_attr() -> Attribute {
    Attribute {
        pound_token: Pound::default(),
        style: AttrStyle::Outer,
        bracket_token: token::Bracket::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: None,
                segments: {
                    let mut punctuated = Punctuated::default();
                    punctuated.push(PathSegment {
                        ident: Ident::new("derive", Span::call_site()),
                        arguments: Default::default(),
                    });
                    punctuated
                },
            },
            delimiter: MacroDelimiter::Paren(token::Paren::default()),
            tokens: quote! {
                askama::Template, yew::Properties, PartialEq
            },
        }),
    }
}