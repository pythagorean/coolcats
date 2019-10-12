use stdweb::{js, web::Node, unstable::TryFrom};
use yew::{prelude::*, virtual_dom::VNode};

pub fn htmlize<T: Component>(text: &str) -> Html<T> {
    let js_html = js! {
        var div = document.createElement("div");
        div.innerHTML = @{text.to_string()};
        return div;
    };
    let node = Node::try_from(js_html).expect("convert js_html");
    VNode::VRef(node)
}
