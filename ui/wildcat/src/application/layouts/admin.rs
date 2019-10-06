use yew::prelude::*;

pub fn admin_wrap<T: Component>(html: Html<T>) -> Html<T> {
    html! {
        <div class = "admin-wrapper">
            {html}
        </div>
    }
}
