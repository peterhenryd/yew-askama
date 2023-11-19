# yew-askama

This crate provides an attribute that allows you to easily turn Askama templates into Yew components.

## Usage

This crate exports one public member: the `template_component` procedural macro attribute. This macro can be applied to
structures, and is minimally invasive, since it does not redefine the struct, but rather edits it.

Take this example:

```rs
#[template_component]
#[template(path = "card.html")
pub struct Card {
    title: &'static str,
    content: &'static str,
}
```
which is (excluding some imports) transformed into:

```rs
#[derive(askama::Template, yew::Properties, PartialEq)]
#[template(path = "card.html")
pub struct CardTemplate {
    title: &'static str,
    content: &'static str,
}

#[function_component]
pub fn Card(template: &CardTemplate) -> Html {
    let template_string = template.render().unwrap();
    let attr_value = AttrValue::from_str(template_string.as_str()).unwrap();
    
    Html::from_html_unchecked(attr_value)
}
```

The `template_component` attribute requires Askama's `template` attribute to be defined. See 
[here](https://djc.github.io/askama/creating_templates.html#the-template-attribute) for more information.

Note that Askama's `Template` trait and Yew's `Properties` trait are derived onto the same type. This means you must
consider the limitations of both traits, such as not being able to use lifetimes.

## Limitations

- **Stateless**: Since the components are defined using a structure, there is currently no way to run any code from the
component in the code. This is not an inherent limitation, as this feature could be implemented (pretty easily) using
implementations.
- **Non-selective visibility**: While not an urgent issue, the visibility for the Yew component is currently not 
configurable.