use yew::prelude::*;

use crate::application::resources;

pub struct Home {
    resources: Box<dyn Bridge<resources::Worker>>,
    value: String,
}

pub enum Msg {
    GetResources,
    Resources(resources::Response),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let resources = resources::Worker::bridge(link.send_back(Msg::Resources));
        let mut component = Self { resources, value: String::new() };
        component.update(Msg::GetResources);
        component
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetResources => {
                self.resources.send(resources::Request::LocaleText("compose_form-placeholder".into()));
            }
            Msg::Resources(response) => match response {
                resources::Response::LocaleText(value) => {
                    self.value = value;
                    return true;
                }
            }
        }
        false
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{&self.value}</p>
        }
    }
}
