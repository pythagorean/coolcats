use stdweb::{js, web::Node, unstable::TryFrom};
use yew::{prelude::*, virtual_dom::VNode};

pub fn svg_logo_full<T: Component>() -> Html<T> {
    //content_tag(:svg, tag(:use, 'xlink:href' => '#mastodon-svg-logo-full'), 'viewBox' => '0 0 713.35878 175.8678')
    const LOGO_FULL: &str = include_str!("../resources/images/logo_full.svg");
    const SVG: &str = r##"
        <svg viewbox = "0 0 713.35878 175.8678">
            <use href = "#mastodon-svg-logo-full"/>
        </svg>
    "##;
    let js_svg = js! {
        var div = document.createElement("div");
        div.innerHTML = @{LOGO_FULL.to_string() + SVG};
        return div;
    };
    let node = Node::try_from(js_svg).expect("convert js_svg");
    VNode::VRef(node)
}
