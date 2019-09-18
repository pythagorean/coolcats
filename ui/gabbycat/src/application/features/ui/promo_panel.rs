use yew::prelude::*;

pub struct PromoPanel;

impl Renderable<PromoPanel> for PromoPanel {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

pub enum Msg {}

impl Component for PromoPanel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
