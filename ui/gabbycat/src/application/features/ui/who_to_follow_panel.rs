use yew::prelude::*;

pub struct WhoToFollowPanel;

impl Renderable<WhoToFollowPanel> for WhoToFollowPanel {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

pub enum Msg {}

impl Component for WhoToFollowPanel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
