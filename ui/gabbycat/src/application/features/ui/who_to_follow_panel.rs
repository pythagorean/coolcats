use yew::prelude::*;

use gabbycat_macros::ImplComponent;

#[derive(ImplComponent)]
pub struct WhoToFollowPanel;

impl Renderable<WhoToFollowPanel> for WhoToFollowPanel {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}
