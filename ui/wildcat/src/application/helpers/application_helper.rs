use yew::prelude::*;

pub fn fa_icon<T: Component>(icon: &str) -> Html<T> {
    let v: Vec<&str> = icon.split(' ').collect();
    let class = format!("fa fa-{}", v.join(" fa-")); // qu'est-ce que c'est?
    html! {
        <i class = class></i>
    }
}
