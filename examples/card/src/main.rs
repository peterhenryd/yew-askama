use yew_askama::template_component;

fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Card title="Title" content="Content" />
    }
}

#[template_component]
#[template(path = "card.html")]
pub struct Card {
    title: &'static str,
    content: &'static str
}