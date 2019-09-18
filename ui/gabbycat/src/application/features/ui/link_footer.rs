use yew::prelude::*;

pub struct LinkFooter;

impl Renderable<LinkFooter> for LinkFooter {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

pub enum Msg {}

impl Component for LinkFooter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
