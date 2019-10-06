use titlecase::titlecase;
use yew::prelude::*;

pub fn content_for<T: Component>(name: &str, html: Html<T>) -> Html<T> {
    html! {
        <div class = "content-wrapper">
            <div class = "content">
                <h2>{titlecase(name).replace("_", " ")}</h2>
                {html}
            </div>
        </div>
    }
}
