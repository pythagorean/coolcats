use yew::prelude::*;

use crate::application::Application;

pub struct Model;

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <Application />
        }
    }
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}
