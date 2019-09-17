use yew::prelude::*;

pub struct Upload;

impl Renderable<Upload> for Upload {
    fn view(&self) -> Html<Self> {
        html! {}
    }
}

pub enum Msg {}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub id: String,
    #[props(required)]
    pub key: String,
}

impl Component for Upload {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
