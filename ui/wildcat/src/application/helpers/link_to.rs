use yew::prelude::*;

pub fn link_to<T: Component>(path: &str, html: Html<T>) -> Html<T> {
    html! {
        <a href = path>
            {html}
        </a>
    }
}
