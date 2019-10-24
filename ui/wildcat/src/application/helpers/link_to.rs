use yew::prelude::*;

use crate::application::routes::RouterTarget;

pub fn link_to_target<T: Component>(target: RouterTarget, html: Html<T>) -> Html<T> {
    let path: &str = target.into();
    html! {
        <a href = path>
            {html}
        </a>
    }
}

pub fn link_to_class<T: Component>(class: &str, html: Html<T>) -> Html<T> {
    html! {
        <a href = "#", class = class>
            {html}
        </a>
    }
}
