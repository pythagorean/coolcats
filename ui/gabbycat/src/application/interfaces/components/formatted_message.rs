use yew::prelude::*;

pub struct FormattedMessage;

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
    #[props(required)]
    pub default_message: String,
}

impl Renderable<FormattedMessage> for FormattedMessage {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

impl Component for FormattedMessage {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}
