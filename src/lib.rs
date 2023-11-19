use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{ItemStruct, parse_macro_input};
use crate::derive_attr::create_derive_attr;

mod derive_attr;

/// Creates a Yew component from an Askama template.
///
/// # Examples
///
/// ```rs
/// #[template_component]
/// #[template(path = "card.html")
/// pub struct Card {
///     title: &'static str,
///     content: &'static str,
/// }
/// ```
///
/// # Requirements
///
/// - The attribute may only be applied to structs.
/// - The structure must have [Askama's template attribute](https://djc.github.io/askama/creating_templates.html#the-template-attribute).
/// - THe structure may not have lifetime parameters.
///
/// # Step-by-step
///
/// This attribute takes in no arguments, and creates a Yew component which renders an Askama
/// template. It does so by appending "Template" to the target structure, deriving
/// "askama::Template" and "yew::Properties", and then defining a new function with the attribute
/// [yew::function_component], named after the original structure.
#[proc_macro_attribute]
pub fn template_component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = parse_macro_input!(input as ItemStruct);

    let mut attrs = vec![create_derive_attr()];
    attrs.append(&mut output.attrs);

    output.attrs = attrs;

    let template_ident_string = format!("{}Template", output.ident.to_string());
    let template_ident_span = output.ident.span();

    let name = output.ident.clone();

    let template_ident = Ident::new(template_ident_string.as_str(), template_ident_span);
    let template_ident_token_stream = template_ident.to_token_stream();

    output.ident = template_ident;

    let mut output_token_stream = output.to_token_stream();

    output_token_stream.append_all(quote! {
        #[yew::function_component]
        pub fn #name(template: &#template_ident_token_stream) -> yew::Html {
            use askama::Template;
            use std::str::FromStr;

            let template_string = template.render().unwrap();
            let attr_value = yew::AttrValue::from_str(template_string.as_str()).unwrap();

            yew::Html::from_html_unchecked(attr_value)
        }
    });

    output_token_stream.into()
}