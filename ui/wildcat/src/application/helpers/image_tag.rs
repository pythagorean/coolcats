use yew::prelude::*;

pub fn image_tag<T: Component>(url: &str, alt: &str) -> Html<T> {
    html! {
        <img src = url, alt = alt/>
    }
}
