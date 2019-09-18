use yew::prelude::*;

pub struct GroupSidebarPanel;

impl Renderable<GroupSidebarPanel> for GroupSidebarPanel {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

pub enum Msg {}

impl Component for GroupSidebarPanel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
