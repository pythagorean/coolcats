use yew::prelude::*;

use super::locales::en;

pub struct Home;

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        let locale = en::Locale::initialize();

        let value = locale.get_value("compose_form-placeholder");

        html! {
            <p>{value}</p>
        }
    }
}
