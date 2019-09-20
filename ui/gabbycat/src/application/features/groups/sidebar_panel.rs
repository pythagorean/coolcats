use yew::prelude::*;

use gabbycat_macros::ImplComponent;

#[derive(ImplComponent)]
pub struct GroupSidebarPanel;

impl Renderable<GroupSidebarPanel> for GroupSidebarPanel {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}
