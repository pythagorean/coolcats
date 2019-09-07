use yew::prelude::*;

use super::locales::en::Ftl;

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
        let ftl = Ftl::new();

        let msg = ftl
            .bundle
            .get_message("compose_form-placeholder")
            .expect("Message doesn't exist.");
        let pattern = msg.value.expect("Message has no value.");

        let mut errors = vec![];
        let value = ftl.bundle.format_pattern(&pattern, None, &mut errors);

        html! {
            <p>{value}</p>
        }
    }
}
